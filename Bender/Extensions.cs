using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace Bender
{
    static class Extensions
    {
        public static T Random<T>(this T[] collection, T @default = default(T))
        {
            if (collection.Length == 0)
                return @default;
            
            var rand = new Random();
            return collection[rand.Next(collection.Length)];
        }

        public static T Random<T>(this IEnumerable<T> collection, T @default = default(T))
        {
            var rand = new Random();
            var count = 0;
            var current = @default;

            foreach(var item in collection)
            {
                if (rand.Next(++count) == 0)
                    current = item;
            }

            return current;
        }
    }
}
