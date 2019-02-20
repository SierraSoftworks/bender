package api_test

import (
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"time"

	"github.com/SierraSoftworks/bender/internal/pkg/providers"
	"github.com/SierraSoftworks/bender/pkg/models"

	. "github.com/onsi/ginkgo"
	. "github.com/onsi/gomega"

	. "github.com/SierraSoftworks/bender/internal/pkg/api"
)

var _ = Describe("Health API", func() {
	quotes := providers.NewQuoteProvider()
	api, err := NewAPI(quotes)

	It("Should initialize correctly", func() {
		Expect(err).To(BeNil())
	})

	ts := httptest.NewServer(api.Handler())
	url := fmt.Sprintf("%s/api/v1/status", ts.URL)

	res, err := http.Get(url)

	It("Should not return an error", func() {
		Expect(err).To(BeNil())
		Expect(res).ToNot(BeNil())
	})

	It("Should return a 200 status code", func() {
		Expect(res).ToNot(BeNil())
		Expect(res.StatusCode).To(Equal(200))
	})

	It("Should return a health object", func() {
		Expect(res).ToNot(BeNil())

		var health models.Health
		Expect(json.NewDecoder(res.Body).Decode(&health)).To(BeNil())
		Expect(health.Started.Location()).To(Equal(time.UTC))
	})
})
