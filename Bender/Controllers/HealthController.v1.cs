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
    [Area("v1")]
    public class HealthV1Controller : HealthController<Health.Version1>
    {
        public HealthV1Controller(HealthStore store, IRepresenter<Health, Health.Version1> representer)
            : base(store, representer)
        {
        }
    }
}