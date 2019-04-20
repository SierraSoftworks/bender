using System;
using System.Runtime.Serialization;

namespace Bender.Models
{
    public class Health
    {
        public DateTime Started { get; set; }

        [DataContract(Name = "Health")]
        public class Version1 : IView<Health>
        {
            [DataMember(Name = "started")]
            public DateTime Started { get; set; }

            public class Representer : IRepresenter<Health, Version1>
            {
                public Health ModelFromView(Version1 view)
                {
                    return new Health
                    {
                        Started = view.Started,
                    };
                }

                public Version1 ViewFromModel(Health model)
                {
                    return new Version1
                    {
                        Started = model.Started,
                    };
                }
            }
        }
    }
}
