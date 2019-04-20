using System;
using System.Runtime.Serialization;

namespace Bender.Models
{
    public class Quote
    {
        public string Text { get; set; }

        public string Who { get; set; }

        [DataContract(Name = "Quote")]
        public class Version1 : IView<Quote>
        {
            [DataMember(Name = "quote", Order = 1)]
            public string Quote { get; set; }

            [DataMember(Name = "who", Order = 2)]
            public string Who { get; set; }

            public class Representer : IRepresenter<Quote, Version1>
            {
                public Quote ModelFromView(Version1 view)
                {
                    return new Quote
                    {
                        Text = view.Quote,
                        Who = view.Who,
                    };
                }

                public Version1 ViewFromModel(Quote model)
                {
                    return new Version1
                    {
                        Quote = model.Text,
                        Who = model.Who,
                    };
                }
            }
        }
    }
}
