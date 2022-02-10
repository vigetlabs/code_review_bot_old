package lang

import (
	"strings"

	"gopkg.in/yaml.v3"
)

var emojis = map[string]string{
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

// A Lang represents a programming language
type Lang struct {
	Name       string
	Group      string
	Extensions []string
	Filenames  []string
}

// Emoji returns the Slack emoji for the language or the empty string
func (l *Lang) Emoji() string {
	return emojis[l.Name]
}

// An Index builds an index of programming languages and their attributes
type Index struct {
	nameIndex     map[string]*Lang
	extIndex      map[string]*Lang
	filenameIndex map[string]*Lang
}

// Load parses YAML data to build the index
func (i *Index) Load(data []byte) error {
	m := make(map[string]*Lang)
	if err := yaml.Unmarshal(data, &m); err != nil {
		return err
	}

	for name, l := range m {
		l.Name = name

		i.nameIndex[name] = l

		for _, s := range l.Extensions {
			ext := s[1:] // Trim leading "."

			// If the ext is used by more than one lang, choose the one with the least extensions
			if cl, ok := i.extIndex[ext]; ok {
				if len(cl.Extensions) <= len(l.Extensions) {
					continue
				}
			}

			i.extIndex[ext] = l
		}

		for _, f := range l.Filenames {
			i.filenameIndex[f] = l
		}
	}

	return nil
}

// LangByName returns the language by name
func (i *Index) LangByName(name string) *Lang {
	return i.nameIndex[name]
}

// LangForFile returns the language for the filename
func (i *Index) LangForFile(filename string) *Lang {
	// If there is an exact filename match, return that language
	if l, ok := i.filenameIndex[filename]; ok {
		return l
	}

	containsDot := strings.Contains(filename, ".")
	if !containsDot {
		return nil
	}

	ps := strings.Split(filename, ".")
	ext := ps[len(ps)-1]

	return i.extIndex[ext]
}

// NewIndex returns a new Index
func NewIndex() *Index {
	return &Index{
		nameIndex:     make(map[string]*Lang),
		extIndex:      make(map[string]*Lang),
		filenameIndex: make(map[string]*Lang),
	}
}
