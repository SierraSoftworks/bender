using Bender.Models;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace Bender.Stores
{
    public class HealthStore
    {
        public DateTime StartTime { get; } = DateTime.UtcNow;

        public Task<Health> GetHealthAsync() => Task.FromResult(new Health
            {
                Started = StartTime
            });
    }
}
