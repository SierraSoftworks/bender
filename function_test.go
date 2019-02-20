package bender_test

import (
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"os"

	. "github.com/onsi/ginkgo"
	. "github.com/onsi/gomega"

	. "github.com/SierraSoftworks/bender"
	"github.com/SierraSoftworks/bender/pkg/models"
)

var _ = Describe("Function as a Service", func() {
	if os.Getenv("GOOGLE_APPLICATION_CREDENTIALS") == "" {
		Skip("Google Cloud credentials are not available")
	}

	ts := httptest.NewServer(http.HandlerFunc(Bender))
	defer ts.Close()

	Describe("Getting a random quote", func() {
		url := fmt.Sprintf("%s/api/v1/quote", ts.URL)
		res, err := http.Get(url)

		It("Should respond without an error", func() {
			Expect(err).To(BeNil())
		})

		It("Should respond with a 200 OK status code", func() {
			Expect(res.StatusCode).To(Equal(200))
		})

		It("Should deserialize into a valid quote", func() {
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
			Expect(res.StatusCode).To(Equal(200))
		})

		It("Should deserialize into a valid quote", func() {
			var quote models.Quote
			Expect(json.NewDecoder(res.Body).Decode(&quote)).To(BeNil())
			Expect(quote.Quote).ToNot(BeEmpty())
			Expect(quote.Who).To(Equal("Bender"))
		})
	})
})
