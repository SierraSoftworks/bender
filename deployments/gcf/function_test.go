package bender_gcf

import (
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"

	. "github.com/onsi/ginkgo"
	. "github.com/onsi/gomega"

	"github.com/SierraSoftworks/bender/pkg/models"
)

var _ = Describe("Function as a Service", func() {
	ts := httptest.NewServer(http.HandlerFunc(Bender))
	defer ts.Close()

	Describe("Service", func() {
		svc := &FunctionService{}
		It("Should initialize without errors", func() {
			Expect(svc.Initialize()).To(BeNil())
		})

		It("Should have created the App provider", func() {
			Expect(svc.App).ToNot(BeNil())
		})
	})

	Describe("Getting a random quote", func() {
		url := fmt.Sprintf("%s/api/v1/quote", ts.URL)
		res, err := http.Get(url)

		It("Should respond without an error", func() {
			Expect(err).To(BeNil())
		})

		It("Should respond with a 200 OK status code", func() {
			Expect(res).ToNot(BeNil())
			Expect(res.StatusCode).To(Equal(200))
		})

		It("Should deserialize into a valid quote", func() {
			Expect(res).ToNot(BeNil())
			var quote models.Quote
			Expect(json.NewDecoder(res.Body).Decode(&quote)).To(BeNil())
			Expect(quote.Quote).ToNot(BeEmpty())
			Expect(quote.Who).ToNot(BeEmpty())
		})
	})

	Describe("Getting a random quote by a specific author", func() {
		url := fmt.Sprintf("%s/api/v1/quote/Bender", ts.URL)
		res, err := http.Get(url)

		It("Should respond without an error", func() {
			Expect(err).To(BeNil())
		})

		It("Should respond with a 200 OK status code", func() {
			Expect(res).ToNot(BeNil())
			Expect(res.StatusCode).To(Equal(200))
		})

		It("Should deserialize into a valid quote", func() {
			Expect(res).ToNot(BeNil())

			var quote models.Quote
			Expect(json.NewDecoder(res.Body).Decode(&quote)).To(BeNil())
			Expect(quote.Quote).ToNot(BeEmpty())
			Expect(quote.Who).To(Equal("Bender"))
		})
	})
})
