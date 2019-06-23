using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Bender.Models;
using Bender.Stores;
using Microsoft.AspNetCore.Cors;
using Microsoft.AspNetCore.Mvc;

namespace Bender.Controllers
{
    public abstract class QuoteController<TView> : ControllerBase
        where TView : IView<Quote>
    {
        public IQuoteStore Store { get; }

        public IRepresenter<Quote, TView> Representer { get; }

        public QuoteController(IQuoteStore store, IRepresenter<Quote, TView> representer)
        {
            this.Store = store;
            this.Representer = representer;
        }

        // GET api/v1/quote
        [HttpGet]
        [Route("api/[area]/quote")]
        [EnableCors]
        public virtual async Task<ActionResult<TView>> Get()
        {
            var quote = await Store.GetQuoteAsync();
            return quote == null ? (ActionResult<TView>)NotFound() : (ActionResult<TView>)Representer.ViewFromModel(quote);
        }

        // GET api/v1/quote/{by}
        [HttpGet]
        [Route("api/[area]/quote/{by}")]
        [EnableCors]
        public virtual async Task<ActionResult<TView>> Get(string by)
        {
            var quote = await Store.GetQuoteByAsync(by);
            return quote == null ? (ActionResult<TView>)NotFound() : (ActionResult<TView>)Representer.ViewFromModel(quote);
        }
    }
}
