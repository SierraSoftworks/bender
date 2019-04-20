using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Threading.Tasks;
using Bender.Config;
using Bender.Models;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Options;
using Newtonsoft.Json;

namespace Bender.Stores
{
    public class FileQuoteStore<TStored> : IQuoteStore
        where TStored : IView<Quote>
    {
        public FileQuoteStore(IOptions<FileQuoteStoreConfig> config, IRepresenter<Quote, TStored> representer, ILogger<FileQuoteStore<TStored>> logger)
        {
            this.config = config.Value;
            this.Representer = representer;
            this.logger = logger;
        }

        private Quote[] quotes;
        private readonly FileQuoteStoreConfig config;
        private readonly ILogger<FileQuoteStore<TStored>> logger;

        public IRepresenter<Quote, TStored> Representer { get; }

        public async Task<Quote> GetQuoteAsync() => (await EnsureQuotesAsync()).Random();

        public async Task<Quote> GetQuoteByAsync(string author) => (await EnsureQuotesAsync()).Where(q => q.Who.Equals(author, StringComparison.InvariantCultureIgnoreCase)).Random();

        private async Task<Quote[]> EnsureQuotesAsync()
        {
            if (quotes == null)
                quotes = await GetQuotesAsync();

            return quotes ?? Array.Empty<Quote>();
        }

        private async Task<Quote[]> GetQuotesAsync()
        {
            try
            {
                using (var file = File.OpenRead(config.QuoteFile))
                using(var sr = new StreamReader(file))
                {
                    var quotes = JsonConvert.DeserializeObject<TStored[]>(await sr.ReadToEndAsync());

                    return quotes.Select(quote => Representer.ModelFromView(quote)).ToArray();
                }
            } catch (Exception ex)
            {
                logger.LogError(ex, "Failed to load quotes file from {QuoteFile}", config.QuoteFile);
                return null;
            }
        }
    }
}
