using Bender.Config;
using Bender.Models;
using Bender.Stores;
using Microsoft.AspNetCore.Mvc.Testing;
using Microsoft.Extensions.Options;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Xunit;

namespace Bender.Tests.Stores
{
    public class MemoryQuoteStoreTests
        : IClassFixture<WebApplicationFactory<Startup>>
    {
        [Fact]
        public async Task TestGetQuoteNoneAvailable()
        {
            var store = new MemoryQuoteStore(Array.Empty<Quote>());

            var quote = await store.GetQuoteAsync();
            Assert.Null(quote);
        }

        [Fact]
        public async Task TestGetQuote()
        {
            var store = new MemoryQuoteStore(new[] {new Quote
            {
                Text = "Bite my shiny metal ass!",
                Who = "Bender"
            }
            });

            var quote = await store.GetQuoteAsync();
            Assert.NotNull(quote);
            Assert.Equal("Bender", quote.Who);
            Assert.Equal("Bite my shiny metal ass!", quote.Text);
        }

        [Fact]
        public async Task TestGetQuoteIsRandom()
        {
            var quotes = new[] {
                new Quote
                {
                    Text = "Bite my shiny metal ass!",
                    Who = "Bender"
                },
                new Quote
                {
                    Text = "Well that went better than expected",
                    Who = "Ben"
                }
            };

            var store = new MemoryQuoteStore(quotes);

            var seenAuthors = new HashSet<string>();

            for (var i = 0; i < 100; i++)
            {
                var quote = await store.GetQuoteAsync();
                Assert.NotNull(quote);
                seenAuthors.Add(quote.Who);
            }

            Assert.Equal(2, seenAuthors.Count);
        }
    }
}
