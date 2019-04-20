using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Bender.Models;
using Bender.Stores;
using Microsoft.AspNetCore.Http;
using Microsoft.AspNetCore.Mvc;

namespace Bender.Controllers
{
    [ApiController]
    [ApiVersion("1.0")]
    public class QuoteV1Controller : QuoteController<Quote.Version1>
    {
        public QuoteV1Controller(IQuoteStore store, IRepresenter<Quote, Quote.Version1> representer)
            : base(store, representer)
        {
        }
    }
}