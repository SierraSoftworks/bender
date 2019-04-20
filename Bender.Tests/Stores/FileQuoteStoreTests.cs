using Bender.Config;
using Bender.Models;
using Bender.Stores;
using Microsoft.AspNetCore.Mvc.Testing;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Options;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Xunit;

namespace Bender.Tests.Stores
{
    public class FileQuoteStoreTests
        : IClassFixture<WebApplicationFactory<Startup>>
    {
        public FileQuoteStoreTests()
        {

        }

        private readonly ILoggerFactory loggerFactory = LoggerFactory.Create(b => b.SetMinimumLevel(LogLevel.Error).AddDebug());

        private readonly FileQuoteStoreConfig config = new FileQuoteStoreConfig();

        protected IQuoteStore Store => new FileQuoteStore<Quote.Version1>(
            Options.Create(config),
            new Quote.Version1.Representer(),
            loggerFactory.CreateLogger<FileQuoteStore<Quote.Version1>>());

        [Fact]
        public async Task TestGetQuoteNoneAvailable()
        {
            config.QuoteFile = "quotes.nonexistent.json";

            var quote = await Store.GetQuoteAsync();
            Assert.Null(quote);
        }

        [Fact]
        public async Task TestGetQuote()
        {
            config.QuoteFile = "quotes.json";

            var quote = await Store.GetQuoteAsync();
            Assert.NotNull(quote);
        }

        [Fact]
        public async Task TestGetQuoteIsRandom()
        {
            config.QuoteFile = "quotes.json";

            var store = Store;

            var seenAuthors = new HashSet<string>();

            for (var i = 0; i < 100; i++)
            {
                var quote = await store.GetQuoteAsync();
                Assert.NotNull(quote);
                seenAuthors.Add(quote.Who);
            }

            Assert.InRange(seenAuthors.Count, 2, 100);
        }
    }
}
