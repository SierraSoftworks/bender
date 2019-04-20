using Bender.Config;
using Bender.Models;
using Bender.Stores;
using Microsoft.AspNetCore.Hosting;
using Microsoft.AspNetCore.Mvc.Testing;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Options;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace Bender.Tests
{
    public class BenderAppFactory : WebApplicationFactory<Startup>
    {
        public Task ClearQuotesAsync()
        {
            return quoteStore.ClearQuotesAsync();
        }

        public Task AddQuoteAsync(Quote quote)
        {
            return quoteStore.AddQuoteAsync(quote);
        }

        private readonly MemoryQuoteStore quoteStore = new MemoryQuoteStore(Array.Empty<Quote>());

        protected override void ConfigureWebHost(IWebHostBuilder builder)
        {
            base.ConfigureWebHost(builder);

            builder.ConfigureServices(services =>
            {
                services.AddSingleton<IQuoteStore>(quoteStore);
            });
        }
    }
}
