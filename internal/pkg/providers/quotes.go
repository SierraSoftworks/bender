package providers

import (
	"bytes"
	"encoding/json"
	"io/ioutil"
	"math/rand"
	"strings"
	"time"

	"github.com/SierraSoftworks/bender/pkg/models"

	sentry "github.com/SierraSoftworks/sentry-go"
	"github.com/pkg/errors"
	log "github.com/sirupsen/logrus"
)

type QuoteProvider struct {
	quotes  []*models.Quote
	randSrc rand.Source
}

func NewQuoteProvider() *QuoteProvider {
	return &QuoteProvider{
		quotes:  []*models.Quote{},
		randSrc: rand.NewSource(time.Now().UnixNano()),
	}
}

func (p *QuoteProvider) AddQuote(quote *models.Quote) {
	p.quotes = append(p.quotes, quote)
}

func (p *QuoteProvider) Load(file string) error {
	sentry.DefaultBreadcrumbs().NewDefault(map[string]interface{}{
		"file": file,
	}).WithCategory("models").WithMessage("Loading quotes file")

	data, err := ioutil.ReadFile(file)
	if err != nil {
		return errors.Wrap(err, "failed to read quotes file")
	}

	sentry.DefaultBreadcrumbs().NewDefault(map[string]interface{}{
		"file":     file,
		"fileSize": len(data),
	}).WithCategory("models").WithMessage("Parsing quotes file")

	quotes := []*models.Quote{}
	buf := bytes.NewBuffer(data)
	if err := json.NewDecoder(buf).Decode(&quotes); err != nil {
		return errors.Wrap(err, "failed to parse quotes file")
	}

	sentry.DefaultBreadcrumbs().NewDefault(map[string]interface{}{
		"file":   file,
		"quotes": len(quotes),
	}).WithCategory("models").WithMessage("Loaded new quotes")

	log.WithField("file", file).Infof("Loaded %d new quotes", len(quotes))

	p.quotes = append(p.quotes, quotes...)

	return nil
}

func (p *QuoteProvider) GetRandom() *models.Quote {
	return p.pickRandom(p.quotes)
}

func (p *QuoteProvider) GetAllBy(who string) []*models.Quote {
	filtered := []*models.Quote{}
	for _, quote := range p.quotes {
		if strings.EqualFold(quote.Who, who) {
			filtered = append(filtered, quote)
		}
	}

	return filtered
}

func (p *QuoteProvider) GetRandomBy(who string) *models.Quote {
	return p.pickRandom(p.GetAllBy(who))
}

func (p *QuoteProvider) pickRandom(slice []*models.Quote) *models.Quote {
	l := len(slice)
	if l == 0 {
		return nil
	}

	return slice[rand.New(p.randSrc).Intn(l)]
}
