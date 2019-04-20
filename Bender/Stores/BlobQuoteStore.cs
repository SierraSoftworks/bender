using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Threading.Tasks;
using Bender.Models;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.Logging;
using Microsoft.WindowsAzure.Storage;
using Microsoft.WindowsAzure.Storage.Blob;
using Newtonsoft.Json;

namespace Bender.Stores
{
    public class BlobQuoteStore<TStored> : IQuoteStore
        where TStored : IView<Quote>
    {
        public BlobQuoteStore(IConfiguration configuration, IRepresenter<Quote, TStored> representer, ILogger<BlobQuoteStore<TStored>> logger)
        {
            var storageAccount = CloudStorageAccount.Parse(configuration.GetConnectionString("BlobStorage"));
            var blobClient = storageAccount.CreateCloudBlobClient();
            blobContainer = blobClient.GetContainerReference("quotes");
            this.Representer = representer;
            this.logger = logger;
        }

        private readonly CloudBlobContainer blobContainer;
        private readonly ILogger<BlobQuoteStore<TStored>> logger;
        private Quote[] quotes;

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
                await blobContainer.CreateIfNotExistsAsync();

                var quotesFile = blobContainer.GetBlockBlobReference("quotes.json");
                if (!await quotesFile.ExistsAsync())
                    return Array.Empty<Quote>();

                using (var stream = await quotesFile.OpenReadAsync())
                using (var sr = new StreamReader(stream))
                {
                    var quotes = JsonConvert.DeserializeObject<TStored[]>(await sr.ReadToEndAsync());

                    return quotes.Select(quote => Representer.ModelFromView(quote)).ToArray();
                }
            }
            catch (Exception ex)
            {
                logger.LogError(ex, "Failed to read quotes from Azure Blob Storage");
                return null;
            }
        }
    }
}
