using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Bender.Models;

namespace Bender.Stores
{
    public class MemoryQuoteStore : IQuoteStore
    {
        public MemoryQuoteStore(IEnumerable<Quote> quotes) => this.quotes = quotes.ToList();

        public Task AddQuoteAsync(Quote quote)
        {
            lock (quotes)
            {
                quotes.Add(quote);
            }

            return Task.CompletedTask;
        }

        public Task ClearQuotesAsync()
        {
            lock (quotes)
            {
                quotes.Clear();
            }
            return Task.CompletedTask;
        }

        private readonly List<Quote> quotes;

        public Task<Quote> GetQuoteAsync() {
            lock (quotes)
            {
                return Task.FromResult(quotes.Random());
            }
        }

        public Task<Quote> GetQuoteByAsync(string author)
        {
            lock (quotes)
            {
                return Task.FromResult(quotes.Where(q => q.Who.Equals(author, StringComparison.InvariantCultureIgnoreCase)).Random());
            }
        }
    }
}
