using Bender.Stores;
using System;
using System.Collections.Generic;
using System.Text;
using System.Threading.Tasks;
using Xunit;

namespace Bender.Tests.Stores
{
    public class HealthStoreTests
    {
        [Fact]
        public async Task TestGetHealth()
        {
            var store = new HealthStore();

            Assert.NotEqual(DateTime.MinValue, store.StartTime);
            var startTime = store.StartTime;

            var model = await store.GetHealthAsync();
            Assert.Equal(startTime, model.Started);
        }
    }
}
