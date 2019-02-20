package loaders_test

import (
	. "github.com/onsi/ginkgo"
	. "github.com/onsi/gomega"

	. "github.com/SierraSoftworks/bender/internal/pkg/loaders"
	. "github.com/SierraSoftworks/bender/test"
)

var _ = Describe("File", func() {
	Describe("NewFileLoader", func() {
		loader := NewFileLoader("/tmp/test.json")
		It("should return a loader", func() {
			Expect(loader).ToNot(BeNil())
		})
	})

	Describe("Load()", func() {
		Describe("When the file does not exist", func() {
			loader := NewFileLoader(GetTestDataPath("nonexistent.json"))
			quotes, err := loader.Load()

			It("should return a nil slice of quotes", func() {
				Expect(quotes).To(BeNil())
			})

			It("should return an error", func() {
				Expect(err).ToNot(BeNil())
			})
		})

		Describe("When the file exists", func() {
			Describe("and it is not valid JSON", func() {
				loader := NewFileLoader(GetTestDataPath("quotes.invalid.txt"))
				quotes, err := loader.Load()

				It("should return a nil slice of quotes", func() {
					Expect(quotes).To(BeNil())
				})

				It("should return an error", func() {
					Expect(err).ToNot(BeNil())
				})
			})

			Describe("and it is empty", func() {
				loader := NewFileLoader(GetTestDataPath("quotes.empty.json"))
				quotes, err := loader.Load()

				It("should not return an error", func() {
					Expect(err).To(BeNil())
				})

				It("should return an empty slice", func() {
					Expect(quotes).ToNot(BeNil())
					Expect(quotes).To(HaveLen(0))
				})
			})

			Describe("and it has quotes", func() {
				loader := NewFileLoader(GetTestDataPath("quotes.one.json"))
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
