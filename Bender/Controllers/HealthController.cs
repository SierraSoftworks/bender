using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Bender.Models;
using Bender.Stores;
using Microsoft.AspNetCore.Mvc;

namespace Bender.Controllers
{
    public abstract class HealthController<TView> : ControllerBase
        where TView : IView<Health>
    {
        public HealthStore Store { get; }
        public IRepresenter<Health, TView> Representer { get; }

        public HealthController(HealthStore store, IRepresenter<Health, TView> representer)
        {
            this.Store = store;
            this.Representer = representer;
        }

        // GET api/v1/health
        [HttpGet]
        [Route("api/[area]/health")]
        public virtual async Task<ActionResult<TView>> Get()
        {
            var health = await Store.GetHealthAsync();

            return Representer.ViewFromModel(health);
        }
    }
}
