using Bender.Models;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace Bender.Stores
{
    public interface IQuoteStore
    {
        Task<Quote> GetQuoteAsync();

        Task<Quote> GetQuoteByAsync(string author);
    }
}
