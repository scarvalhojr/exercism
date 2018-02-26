// Package space calculates age in other planets
package space

type Planet string

var earthOrbitalPeriod float64 = 365.25 * 24 * 60 * 60

var orbitalPeriod = map[Planet]float64{
	"Earth":   earthOrbitalPeriod,
	"Mercury": earthOrbitalPeriod * 0.2408467,
	"Venus":   earthOrbitalPeriod * 0.61519726,
	"Mars":    earthOrbitalPeriod * 1.8808158,
	"Jupiter": earthOrbitalPeriod * 11.862615,
	"Saturn":  earthOrbitalPeriod * 29.447498,
	"Uranus":  earthOrbitalPeriod * 84.016846,
	"Neptune": earthOrbitalPeriod * 164.79132,
}

// Age calculates age in other planets
func Age(ageInSeconds float64, planet Planet) float64 {
	return ageInSeconds / orbitalPeriod[planet]
}
