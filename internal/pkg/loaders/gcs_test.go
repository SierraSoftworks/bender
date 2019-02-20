package loaders_test

import (
	. "github.com/onsi/ginkgo"
	. "github.com/onsi/gomega"

	. "github.com/SierraSoftworks/bender/internal/pkg/loaders"

	"os"
)

var _ = Describe("GCS", func() {
	if os.Getenv("GOOGLE_APPLICATION_CREDENTIALS") == "" {
		Skip("Google Cloud credentials are not available")
	}

	Describe("NewGCSLoader", func() {
		loader := NewGCSLoader("cdn.sierrasoftworks.com", "bender/quotes.json")
		It("should return a loader", func() {
			Expect(loader).ToNot(BeNil())
		})
	})

	Describe("Load", func() {
		Describe("When the bucket does not exist", func() {
			loader := NewGCSLoader("invalid-cdn.sierrasoftworks.com", "bender/quotes.json")
			quotes, err := loader.Load()

			It("should return a nil slice of quotes", func() {
				Expect(quotes).To(BeNil())
			})

			It("should return an error", func() {
				Expect(err).ToNot(BeNil())
			})
		})

		Describe("When the bucket exists", func() {
			Describe("but the file does not", func() {
				loader := NewGCSLoader("cdn.sierrasoftworks.com", "bender/missing.json")
				quotes, err := loader.Load()

				It("should return a nil slice of quotes", func() {
					Expect(quotes).To(BeNil())
				})

				It("should return an error", func() {
					Expect(err).ToNot(BeNil())
				})
			})

			Describe("and the file exists", func() {
				Describe("but it is not valid JSON", func() {
					loader := NewGCSLoader("cdn.sierrasoftworks.com", "bender/quotes.invalid.txt")
					quotes, err := loader.Load()

					It("should return a nil slice of quotes", func() {
						Expect(quotes).To(BeNil())
					})

					It("should return an error", func() {
						Expect(err).ToNot(BeNil())
					})
				})

				Describe("and it is empty", func() {
					loader := NewGCSLoader("cdn.sierrasoftworks.com", "bender/quotes.empty.json")
					quotes, err := loader.Load()

					It("should not return an error", func() {
						Expect(err).To(BeNil())
					})

					It("should return an empty slice", func() {
						Expect(quotes).ToNot(BeNil())
						Expect(quotes).To(HaveLen(0))
					})
				})

				Describe("and it has quotes in it", func() {
					loader := NewGCSLoader("cdn.sierrasoftworks.com", "bender/quotes.one.json")
					quotes, err := loader.Load()

					It("should not return an error", func() {
						Expect(err).To(BeNil())
					})

					It("should return the quotes", func() {
						Expect(quotes).ToNot(BeNil())
						Expect(quotes).To(HaveLen(1))
					})

				})
			})

		})
	})
})
