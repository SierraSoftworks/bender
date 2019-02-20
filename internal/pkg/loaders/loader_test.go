package loaders_test

import (
	. "github.com/onsi/ginkgo"
	. "github.com/onsi/gomega"

	. "github.com/SierraSoftworks/bender/internal/pkg/loaders"
)

var _ = Describe("Loader", func() {
	Describe("New", func() {
		Describe("Relative URLs", func() {
			It("should return a file loader", func() {
				loader := New("/tmp/file.json")
				Expect(loader).ToNot(BeNil())
				Expect(loader).To(BeAssignableToTypeOf(NewFileLoader("/tmp/file.json")))
				Expect(loader.String()).To(Equal("file:///tmp/file.json"))
			})
		})

		Describe("file:// URLs", func() {
			It("should return a file loader", func() {
				loader := New("file:///tmp/file.json")
				Expect(loader).ToNot(BeNil())
				Expect(loader).To(BeAssignableToTypeOf(NewFileLoader("/tmp/file.json")))
				Expect(loader.String()).To(Equal("file:///tmp/file.json"))
			})
		})

		Describe("http:// URLs", func() {
			It("should return an HTTP loader", func() {
				loader := New("http://example.com/file.json")
				Expect(loader).ToNot(BeNil())
				Expect(loader).To(BeAssignableToTypeOf(NewHttpLoader("http://example.com/file.json")))
				Expect(loader.String()).To(Equal("http://example.com/file.json"))
			})
		})

		Describe("https:// URLs", func() {
			It("should return an HTTP loader", func() {
				loader := New("https://example.com/file.json")
				Expect(loader).ToNot(BeNil())
				Expect(loader).To(BeAssignableToTypeOf(NewHttpLoader("https://example.com/file.json")))
				Expect(loader.String()).To(Equal("https://example.com/file.json"))
			})
		})

		Describe("gs:// URLs", func() {
			It("should return a GCS loader", func() {
				loader := New("gs://mybucket/file.json")
				Expect(loader).ToNot(BeNil())
				Expect(loader).To(BeAssignableToTypeOf(NewGCSLoader("mybucket", "file.json")))
				Expect(loader.String()).To(Equal("gs://mybucket/file.json"))
			})
		})
	})
})
