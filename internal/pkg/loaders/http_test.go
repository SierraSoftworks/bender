package loaders_test

import (
	. "github.com/onsi/ginkgo"
	. "github.com/onsi/gomega"

	. "github.com/SierraSoftworks/bender/internal/pkg/loaders"
)

var _ = Describe("Http", func() {
	Describe("NewHttpLoader", func() {
		loader := NewHttpLoader("https://raw.githubusercontent.com/SierraSoftworks/bender/master/configs/quotes.json")
		It("should return a loader", func() {
			Expect(loader).ToNot(BeNil())
		})
	})

	Describe("Load()", func() {
		Describe("When the URL does not exist", func() {
			Describe("because the host is invalid", func() {
				loader := NewHttpLoader("https://non-existent.example.com/non/existent/path/quotes.json")
				quotes, err := loader.Load()

				It("should not return any quotes", func() {
					Expect(quotes).To(BeNil())
				})

				It("should return an error", func() {
					Expect(err).ToNot(BeNil())
				})
			})

			Describe("because the file is missing", func() {
				loader := NewHttpLoader("https://raw.githubusercontent.com/SierraSoftworks/bender/master/non_existent.json")
				quotes, err := loader.Load()

				It("should not return any quotes", func() {
					Expect(quotes).To(BeNil())
				})

				It("should return an error", func() {
					Expect(err).ToNot(BeNil())
				})
			})
		})

		Describe("When the file exists", func() {

			Describe("and it is not valid JSON", func() {
				loader := NewHttpLoader("https://raw.githubusercontent.com/SierraSoftworks/bender/master/test/data/quotes.invalid.txt")
				quotes, err := loader.Load()

				It("should not return any quotes", func() {
					Expect(quotes).To(BeNil())
				})

				It("should return an error", func() {
					Expect(err).ToNot(BeNil())
				})
			})

			Describe("and it is empty", func() {
				loader := NewHttpLoader("https://raw.githubusercontent.com/SierraSoftworks/bender/master/test/data/quotes.empty.json")
				quotes, err := loader.Load()

				It("should return an empty slice", func() {
					Expect(quotes).ToNot(BeNil())
					Expect(quotes).To(HaveLen(0))
				})

				It("should not return an error", func() {
					Expect(err).To(BeNil())
				})
			})

			Describe("and it has quotes", func() {
				loader := NewHttpLoader("https://raw.githubusercontent.com/SierraSoftworks/bender/master/test/data/quotes.one.json")
				quotes, err := loader.Load()

				It("should return the quotes", func() {
					Expect(quotes).ToNot(BeNil())
					Expect(quotes).To(HaveLen(1))
				})

				It("should not return an error", func() {
					Expect(err).To(BeNil())
				})
			})
		})
	})
})
