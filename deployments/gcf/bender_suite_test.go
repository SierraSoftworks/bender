package bender

import (
	"testing"

	. "github.com/onsi/ginkgo"
	. "github.com/onsi/gomega"
)

func TestBender(t *testing.T) {
	RegisterFailHandler(Fail)
	RunSpecs(t, "Bender Suite")
}
