package languages

import "gopkg.in/yaml.v3"

// Emojis maps language names to Slack emojis
var Emojis = map[string]string{
	"CSS":        ":css:",
	"HTML":       ":html:",
	"JavaScript": ":js:",
	"JSON":       ":json:",
	"JSX":        ":react:",
	"Ruby":       ":ruby:",
	"Rust":       ":rust:",
	"Shell":      ":terminal:",
	"TypeScript": ":typescript:",
}

// A Language holds characteristics of a programming language
type Language struct {
	Name       string
	Group      string
	Extensions []string
}

// Parse parses a list of languages from a YAML file
func Parse(data []byte) ([]*Language, error) {
	m := make(map[string]*Language)
	if err := yaml.Unmarshal(data, &m); err != nil {
		return nil, err
	}

	ls := make([]*Language, len(m))
	i := 0
	for n, l := range m {
		l.Name = n
		ls[i] = l
		i++
	}

	return ls, nil
}
