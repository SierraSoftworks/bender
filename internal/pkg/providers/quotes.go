package providers

import (
	"math/rand"
	"strings"
	"time"

	"github.com/SierraSoftworks/bender/internal/pkg/loaders"
	"github.com/SierraSoftworks/bender/pkg/models"

	sentry "github.com/SierraSoftworks/sentry-go"
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

func (p *QuoteProvider) Load(loader loaders.Loader) error {
	quotes, err := loader.Load()
	if err != nil {
		log.WithError(err).Error("Failed to load quotes")
		return err
	}

	sentry.DefaultBreadcrumbs().NewDefault(map[string]interface{}{
		"quotes": len(quotes),
	}).WithCategory("models").WithMessage("Loaded new quotes")

	log.Infof("Loaded %d new quotes", len(quotes))

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
