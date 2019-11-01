CREATE TABLE icon_mappings (
    id SERIAL PRIMARY KEY,
    file_type VARCHAR NOT NULL,
    image_file VARCHAR NOT NULL
);

CREATE TABLE file_extensions (
    id SERIAL PRIMARY KEY,
    extension VARCHAR NOT NULL,
    icon_mapping_id INTEGER REFERENCES icon_mappings(id)
);

CREATE TABLE file_names (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    icon_mapping_id INTEGER REFERENCES icon_mappings(id)
);

CREATE UNIQUE INDEX extension_idx ON file_extensions(extension);
CREATE UNIQUE INDEX name_idx ON file_names(name);
CREATE UNIQUE INDEX file_type_idx ON icon_mappings(file_type);

-- argdown
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('argdown', 'argdown.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.ad',
    (SELECT id FROM icon_mappings WHERE file_type = 'argdown')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.adown',
    (SELECT id FROM icon_mappings WHERE file_type = 'argdown')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.argdown',
    (SELECT id FROM icon_mappings WHERE file_type = 'argdown')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.argdn',
    (SELECT id FROM icon_mappings WHERE file_type = 'argdown')
  );

-- bat
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('bat', 'windows.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.bat',
    (SELECT id FROM icon_mappings WHERE file_type = 'bat')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.cmd',
    (SELECT id FROM icon_mappings WHERE file_type = 'bat')
  );

-- c
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('c', 'c.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.c',
    (SELECT id FROM icon_mappings WHERE file_type = 'c')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.i',
    (SELECT id FROM icon_mappings WHERE file_type = 'c')
  );

-- clojure
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('clojure', 'clojure.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.clj',
    (SELECT id FROM icon_mappings WHERE file_type = 'clojure')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.cljs',
    (SELECT id FROM icon_mappings WHERE file_type = 'clojure')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.cljc',
    (SELECT id FROM icon_mappings WHERE file_type = 'clojure')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.cljx',
    (SELECT id FROM icon_mappings WHERE file_type = 'clojure')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.clojure',
    (SELECT id FROM icon_mappings WHERE file_type = 'clojure')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.edn',
    (SELECT id FROM icon_mappings WHERE file_type = 'clojure')
  );

-- coffeescript
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('coffeescript', 'coffee.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.coffee',
    (SELECT id FROM icon_mappings WHERE file_type = 'coffeescript')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.cson',
    (SELECT id FROM icon_mappings WHERE file_type = 'coffeescript')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.iced',
    (SELECT id FROM icon_mappings WHERE file_type = 'coffeescript')
  );

-- cpp
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('cpp', 'cpp.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.cpp',
    (SELECT id FROM icon_mappings WHERE file_type = 'cpp')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.cc',
    (SELECT id FROM icon_mappings WHERE file_type = 'cpp')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.cxx',
    (SELECT id FROM icon_mappings WHERE file_type = 'cpp')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.c++',
    (SELECT id FROM icon_mappings WHERE file_type = 'cpp')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.hpp',
    (SELECT id FROM icon_mappings WHERE file_type = 'cpp')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.hh',
    (SELECT id FROM icon_mappings WHERE file_type = 'cpp')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.hxx',
    (SELECT id FROM icon_mappings WHERE file_type = 'cpp')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.ii',
    (SELECT id FROM icon_mappings WHERE file_type = 'cpp')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.ino',
    (SELECT id FROM icon_mappings WHERE file_type = 'cpp')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.inl',
    (SELECT id FROM icon_mappings WHERE file_type = 'cpp')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.ipp',
    (SELECT id FROM icon_mappings WHERE file_type = 'cpp')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.hpp.in',
    (SELECT id FROM icon_mappings WHERE file_type = 'cpp')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.h.in',
    (SELECT id FROM icon_mappings WHERE file_type = 'cpp')
  );

-- csharp
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('csharp', 'c-sharp.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.cs',
    (SELECT id FROM icon_mappings WHERE file_type = 'csharp')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.csx',
    (SELECT id FROM icon_mappings WHERE file_type = 'csharp')
  );

-- css
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('css', 'css.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.css',
    (SELECT id FROM icon_mappings WHERE file_type = 'css')
  );

-- dockerfile
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('dockerfile', 'docker.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.dockerfile',
    (SELECT id FROM icon_mappings WHERE file_type = 'dockerfile')
  );
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'dockerfile',
    (SELECT id FROM icon_mappings WHERE file_type = 'dockerfile')
  );

-- elixir
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('elixir', 'elixir.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.ex',
    (SELECT id FROM icon_mappings WHERE file_type = 'elixir')
  );

-- elm
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('elm', 'elm.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.elm',
    (SELECT id FROM icon_mappings WHERE file_type = 'elm')
  );

-- erb
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('erb', 'html_erb.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.erb',
    (SELECT id FROM icon_mappings WHERE file_type = 'erb')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.rhtml',
    (SELECT id FROM icon_mappings WHERE file_type = 'erb')
  );

-- fsharp
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('fsharp', 'f-sharp.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.fs',
    (SELECT id FROM icon_mappings WHERE file_type = 'fsharp')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.fsi',
    (SELECT id FROM icon_mappings WHERE file_type = 'fsharp')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.fsx',
    (SELECT id FROM icon_mappings WHERE file_type = 'fsharp')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.fsscript',
    (SELECT id FROM icon_mappings WHERE file_type = 'fsharp')
  );

-- go
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('go', 'go2.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.go',
    (SELECT id FROM icon_mappings WHERE file_type = 'go')
  );

-- gradle
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('gradle', 'gradle.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.gradle',
    (SELECT id FROM icon_mappings WHERE file_type = 'gradle')
  );

-- groovy
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('groovy', 'grails.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.groovy',
    (SELECT id FROM icon_mappings WHERE file_type = 'groovy')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.gvy',
    (SELECT id FROM icon_mappings WHERE file_type = 'groovy')
  );
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'jenkinsfile',
    (SELECT id FROM icon_mappings WHERE file_type = 'jenkinsfile')
  );

-- haml
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('haml', 'haml.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.haml',
    (SELECT id FROM icon_mappings WHERE file_type = 'haml')
  );

-- handlebars
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('handlebars', 'mustache.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.handlebars',
    (SELECT id FROM icon_mappings WHERE file_type = 'handlebars')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.hbs',
    (SELECT id FROM icon_mappings WHERE file_type = 'handlebars')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.hjs',
    (SELECT id FROM icon_mappings WHERE file_type = 'handlebars')
  );

-- haskell
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('haskell', 'haskell.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.hs',
    (SELECT id FROM icon_mappings WHERE file_type = 'haskell')
  );

-- haxe
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('haxe', 'haxe.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.hx',
    (SELECT id FROM icon_mappings WHERE file_type = 'haxe')
  );

-- html
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('html', 'html.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.html',
    (SELECT id FROM icon_mappings WHERE file_type = 'html')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.htm',
    (SELECT id FROM icon_mappings WHERE file_type = 'html')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.shtml',
    (SELECT id FROM icon_mappings WHERE file_type = 'html')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.xhtml',
    (SELECT id FROM icon_mappings WHERE file_type = 'html')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.mdoc',
    (SELECT id FROM icon_mappings WHERE file_type = 'html')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.jsp',
    (SELECT id FROM icon_mappings WHERE file_type = 'html')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.asp',
    (SELECT id FROM icon_mappings WHERE file_type = 'html')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.aspx',
    (SELECT id FROM icon_mappings WHERE file_type = 'html')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.jshtm',
    (SELECT id FROM icon_mappings WHERE file_type = 'html')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.volt',
    (SELECT id FROM icon_mappings WHERE file_type = 'html')
  );

-- jade
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('jade', 'jade.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.jade',
    (SELECT id FROM icon_mappings WHERE file_type = 'jade')
  );

-- java
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('java', 'java.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.java',
    (SELECT id FROM icon_mappings WHERE file_type = 'java')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.jav',
    (SELECT id FROM icon_mappings WHERE file_type = 'java')
  );

-- javascript
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('javascript', 'javascript.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.js',
    (SELECT id FROM icon_mappings WHERE file_type = 'javascript')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.es6',
    (SELECT id FROM icon_mappings WHERE file_type = 'javascript')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.mjs',
    (SELECT id FROM icon_mappings WHERE file_type = 'javascript')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.pac',
    (SELECT id FROM icon_mappings WHERE file_type = 'javascript')
  );
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'jakefile',
    (SELECT id FROM icon_mappings WHERE file_type = 'jakefile')
  );

-- javascriptreact
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('javascriptreact', 'react.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.jsx',
    (SELECT id FROM icon_mappings WHERE file_type = 'javascriptreact')
  );

-- jinja
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('jinja', 'jinja.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.jinja',
    (SELECT id FROM icon_mappings WHERE file_type = 'jinja')
  );

-- json
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('json', 'json.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.json',
    (SELECT id FROM icon_mappings WHERE file_type = 'json')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.bowerrc',
    (SELECT id FROM icon_mappings WHERE file_type = 'json')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.jshintrc',
    (SELECT id FROM icon_mappings WHERE file_type = 'json')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.jscsrc',
    (SELECT id FROM icon_mappings WHERE file_type = 'json')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.swcrc',
    (SELECT id FROM icon_mappings WHERE file_type = 'json')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.webmanifest',
    (SELECT id FROM icon_mappings WHERE file_type = 'json')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.har',
    (SELECT id FROM icon_mappings WHERE file_type = 'json')
  );
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'composer.lock',
    (SELECT id FROM icon_mappings WHERE file_type = 'composer.lock')
  );
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    '.watchmanconfig',
    (SELECT id FROM icon_mappings WHERE file_type = '.watchmanconfig')
  );
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    '.ember-cli',
    (SELECT id FROM icon_mappings WHERE file_type = '.ember-cli')
  );

-- kotlin
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('kotlin', 'kotlin.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.kt',
    (SELECT id FROM icon_mappings WHERE file_type = 'kotlin')
  );

-- less
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('less', 'less.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.less',
    (SELECT id FROM icon_mappings WHERE file_type = 'less')
  );

-- lua
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('lua', 'lua.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.lua',
    (SELECT id FROM icon_mappings WHERE file_type = 'lua')
  );

-- makefile
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('makefile', 'makefile.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.mk',
    (SELECT id FROM icon_mappings WHERE file_type = 'makefile')
  );
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'makefile',
    (SELECT id FROM icon_mappings WHERE file_type = 'makefile')
  );
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'gnumakefile',
    (SELECT id FROM icon_mappings WHERE file_type = 'gnumakefile')
  );
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'ocamlmakefile',
    (SELECT id FROM icon_mappings WHERE file_type = 'ocamlmakefile')
  );

-- markdown
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('markdown', 'markdown.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.md',
    (SELECT id FROM icon_mappings WHERE file_type = 'markdown')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.mkd',
    (SELECT id FROM icon_mappings WHERE file_type = 'markdown')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.mdwn',
    (SELECT id FROM icon_mappings WHERE file_type = 'markdown')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.mdown',
    (SELECT id FROM icon_mappings WHERE file_type = 'markdown')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.markdown',
    (SELECT id FROM icon_mappings WHERE file_type = 'markdown')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.markdn',
    (SELECT id FROM icon_mappings WHERE file_type = 'markdown')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.mdtxt',
    (SELECT id FROM icon_mappings WHERE file_type = 'markdown')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.mdtext',
    (SELECT id FROM icon_mappings WHERE file_type = 'markdown')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.workbook',
    (SELECT id FROM icon_mappings WHERE file_type = 'markdown')
  );

-- mustache
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('mustache', 'mustache.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.mustache',
    (SELECT id FROM icon_mappings WHERE file_type = 'mustache')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.mst',
    (SELECT id FROM icon_mappings WHERE file_type = 'mustache')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.mu',
    (SELECT id FROM icon_mappings WHERE file_type = 'mustache')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.stache',
    (SELECT id FROM icon_mappings WHERE file_type = 'mustache')
  );

-- nunjucks
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('nunjucks', 'nunjucks.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.nunjucks',
    (SELECT id FROM icon_mappings WHERE file_type = 'nunjucks')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.nunjs',
    (SELECT id FROM icon_mappings WHERE file_type = 'nunjucks')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.nunj',
    (SELECT id FROM icon_mappings WHERE file_type = 'nunjucks')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.nj',
    (SELECT id FROM icon_mappings WHERE file_type = 'nunjucks')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.njk',
    (SELECT id FROM icon_mappings WHERE file_type = 'nunjucks')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.tmpl',
    (SELECT id FROM icon_mappings WHERE file_type = 'nunjucks')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.tpl',
    (SELECT id FROM icon_mappings WHERE file_type = 'nunjucks')
  );

-- objective-c
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('objective-c', 'c.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.m',
    (SELECT id FROM icon_mappings WHERE file_type = 'objective-c')
  );

-- objective-cpp
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('objective-cpp', 'cpp.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.mm',
    (SELECT id FROM icon_mappings WHERE file_type = 'objective-cpp')
  );

-- ocaml
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('ocaml', 'ocaml.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.ml',
    (SELECT id FROM icon_mappings WHERE file_type = 'ocaml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.mli',
    (SELECT id FROM icon_mappings WHERE file_type = 'ocaml')
  );

-- perl
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('perl', 'perl.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.pl',
    (SELECT id FROM icon_mappings WHERE file_type = 'perl')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.pm',
    (SELECT id FROM icon_mappings WHERE file_type = 'perl')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.pod',
    (SELECT id FROM icon_mappings WHERE file_type = 'perl')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.t',
    (SELECT id FROM icon_mappings WHERE file_type = 'perl')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.psgi',
    (SELECT id FROM icon_mappings WHERE file_type = 'perl')
  );

-- php
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('php', 'php.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.php',
    (SELECT id FROM icon_mappings WHERE file_type = 'php')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.php4',
    (SELECT id FROM icon_mappings WHERE file_type = 'php')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.php5',
    (SELECT id FROM icon_mappings WHERE file_type = 'php')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.phtml',
    (SELECT id FROM icon_mappings WHERE file_type = 'php')
  );

-- powershell
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('powershell', 'powershell.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.ps1',
    (SELECT id FROM icon_mappings WHERE file_type = 'powershell')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.psm1',
    (SELECT id FROM icon_mappings WHERE file_type = 'powershell')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.psd1',
    (SELECT id FROM icon_mappings WHERE file_type = 'powershell')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.pssc',
    (SELECT id FROM icon_mappings WHERE file_type = 'powershell')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.psrc',
    (SELECT id FROM icon_mappings WHERE file_type = 'powershell')
  );

-- properties
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('properties', 'java.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.properties',
    (SELECT id FROM icon_mappings WHERE file_type = 'properties')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.directory',
    (SELECT id FROM icon_mappings WHERE file_type = 'properties')
  );
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    '.gitattributes',
    (SELECT id FROM icon_mappings WHERE file_type = '.gitattributes')
  );
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    '.gitconfig',
    (SELECT id FROM icon_mappings WHERE file_type = '.gitconfig')
  );
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'gitconfig',
    (SELECT id FROM icon_mappings WHERE file_type = 'gitconfig')
  );
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    '.gitmodules',
    (SELECT id FROM icon_mappings WHERE file_type = '.gitmodules')
  );
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    '.editorconfig',
    (SELECT id FROM icon_mappings WHERE file_type = '.editorconfig')
  );

-- python
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('python', 'python.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.py',
    (SELECT id FROM icon_mappings WHERE file_type = 'python')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.rpy',
    (SELECT id FROM icon_mappings WHERE file_type = 'python')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.pyw',
    (SELECT id FROM icon_mappings WHERE file_type = 'python')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.cpy',
    (SELECT id FROM icon_mappings WHERE file_type = 'python')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.gyp',
    (SELECT id FROM icon_mappings WHERE file_type = 'python')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.gypi',
    (SELECT id FROM icon_mappings WHERE file_type = 'python')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.pyi',
    (SELECT id FROM icon_mappings WHERE file_type = 'python')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.ipy',
    (SELECT id FROM icon_mappings WHERE file_type = 'python')
  );
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'snakefile',
    (SELECT id FROM icon_mappings WHERE file_type = 'snakefile')
  );

-- r
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('r', 'R.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.r',
    (SELECT id FROM icon_mappings WHERE file_type = 'r')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.rhistory',
    (SELECT id FROM icon_mappings WHERE file_type = 'r')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.rprofile',
    (SELECT id FROM icon_mappings WHERE file_type = 'r')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.rt',
    (SELECT id FROM icon_mappings WHERE file_type = 'r')
  );

-- razor
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('razor', 'html.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.cshtml',
    (SELECT id FROM icon_mappings WHERE file_type = 'razor')
  );

-- ruby
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('ruby', 'ruby.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.rb',
    (SELECT id FROM icon_mappings WHERE file_type = 'ruby')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.rbx',
    (SELECT id FROM icon_mappings WHERE file_type = 'ruby')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.rjs',
    (SELECT id FROM icon_mappings WHERE file_type = 'ruby')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.gemspec',
    (SELECT id FROM icon_mappings WHERE file_type = 'ruby')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.rake',
    (SELECT id FROM icon_mappings WHERE file_type = 'ruby')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.ru',
    (SELECT id FROM icon_mappings WHERE file_type = 'ruby')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.podspec',
    (SELECT id FROM icon_mappings WHERE file_type = 'ruby')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.rbi',
    (SELECT id FROM icon_mappings WHERE file_type = 'ruby')
  );
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'rakefile',
    (SELECT id FROM icon_mappings WHERE file_type = 'rakefile')
  );
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'gemfile',
    (SELECT id FROM icon_mappings WHERE file_type = 'gemfile')
  );
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'guardfile',
    (SELECT id FROM icon_mappings WHERE file_type = 'guardfile')
  );
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'podfile',
    (SELECT id FROM icon_mappings WHERE file_type = 'podfile')
  );
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'capfile',
    (SELECT id FROM icon_mappings WHERE file_type = 'capfile')
  );

-- rust
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('rust', 'rust.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.rs',
    (SELECT id FROM icon_mappings WHERE file_type = 'rust')
  );

-- sass
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('sass', 'sass.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.sass',
    (SELECT id FROM icon_mappings WHERE file_type = 'sass')
  );

-- scss
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('scss', 'sass.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.scss',
    (SELECT id FROM icon_mappings WHERE file_type = 'scss')
  );

-- shellscript
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('shellscript', 'shell.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.sh',
    (SELECT id FROM icon_mappings WHERE file_type = 'shellscript')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.bash',
    (SELECT id FROM icon_mappings WHERE file_type = 'shellscript')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.bashrc',
    (SELECT id FROM icon_mappings WHERE file_type = 'shellscript')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.bash_aliases',
    (SELECT id FROM icon_mappings WHERE file_type = 'shellscript')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.bash_profile',
    (SELECT id FROM icon_mappings WHERE file_type = 'shellscript')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.bash_login',
    (SELECT id FROM icon_mappings WHERE file_type = 'shellscript')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.ebuild',
    (SELECT id FROM icon_mappings WHERE file_type = 'shellscript')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.install',
    (SELECT id FROM icon_mappings WHERE file_type = 'shellscript')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.profile',
    (SELECT id FROM icon_mappings WHERE file_type = 'shellscript')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.bash_logout',
    (SELECT id FROM icon_mappings WHERE file_type = 'shellscript')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.zsh',
    (SELECT id FROM icon_mappings WHERE file_type = 'shellscript')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.zshrc',
    (SELECT id FROM icon_mappings WHERE file_type = 'shellscript')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.zprofile',
    (SELECT id FROM icon_mappings WHERE file_type = 'shellscript')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.zlogin',
    (SELECT id FROM icon_mappings WHERE file_type = 'shellscript')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.zlogout',
    (SELECT id FROM icon_mappings WHERE file_type = 'shellscript')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.zshenv',
    (SELECT id FROM icon_mappings WHERE file_type = 'shellscript')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.zsh-theme',
    (SELECT id FROM icon_mappings WHERE file_type = 'shellscript')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.ksh',
    (SELECT id FROM icon_mappings WHERE file_type = 'shellscript')
  );
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'apkbuild',
    (SELECT id FROM icon_mappings WHERE file_type = 'apkbuild')
  );
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'pkgbuild',
    (SELECT id FROM icon_mappings WHERE file_type = 'pkgbuild')
  );

-- sql
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('sql', 'db.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.sql',
    (SELECT id FROM icon_mappings WHERE file_type = 'sql')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.dsql',
    (SELECT id FROM icon_mappings WHERE file_type = 'sql')
  );

-- stylus
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('stylus', 'stylus.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.styl',
    (SELECT id FROM icon_mappings WHERE file_type = 'stylus')
  );

-- swift
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('swift', 'swift.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.swift',
    (SELECT id FROM icon_mappings WHERE file_type = 'swift')
  );

-- terraform
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('terraform', 'terraform.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.tf',
    (SELECT id FROM icon_mappings WHERE file_type = 'terraform')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.tfvars',
    (SELECT id FROM icon_mappings WHERE file_type = 'terraform')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.hcl',
    (SELECT id FROM icon_mappings WHERE file_type = 'terraform')
  );

-- todo
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('todo', 'todo.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'todo',
    (SELECT id FROM icon_mappings WHERE file_type = 'todo')
  );

-- typescript
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('typescript', 'typescript.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.ts',
    (SELECT id FROM icon_mappings WHERE file_type = 'typescript')
  );

-- typescriptreact
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('typescriptreact', 'react.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.tsx',
    (SELECT id FROM icon_mappings WHERE file_type = 'typescriptreact')
  );

-- vala
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('vala', 'vala.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.vala',
    (SELECT id FROM icon_mappings WHERE file_type = 'vala')
  );

-- vue
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('vue', 'vue.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.vue',
    (SELECT id FROM icon_mappings WHERE file_type = 'vue')
  );

-- xml
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('xml', 'xml.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.xml',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.xsd',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.ascx',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.atom',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.axml',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.bpmn',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.cpt',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.csl',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.csproj',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.csproj.user',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.dita',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.ditamap',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.dtd',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.ent',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.mod',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.dtml',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.fsproj',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.fxml',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.iml',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.isml',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.jmx',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.launch',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.menu',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.mxml',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.nuspec',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.opml',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.owl',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.proj',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.props',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.pt',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.publishsettings',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.pubxml',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.pubxml.user',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.rdf',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.rng',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.rss',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.shproj',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.storyboard',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.targets',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.tld',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.tmx',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.vbproj',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.vbproj.user',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.vcxproj',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.vcxproj.filters',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.wsdl',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.wxi',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.wxl',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.wxs',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.xaml',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.xbl',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.xib',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.xlf',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.xliff',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.xpdl',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.xul',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.xoml',
    (SELECT id FROM icon_mappings WHERE file_type = 'xml')
  );

-- yaml
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('yaml', 'yml.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.yml',
    (SELECT id FROM icon_mappings WHERE file_type = 'yaml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.eyaml',
    (SELECT id FROM icon_mappings WHERE file_type = 'yaml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.eyml',
    (SELECT id FROM icon_mappings WHERE file_type = 'yaml')
  );
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.yaml',
    (SELECT id FROM icon_mappings WHERE file_type = 'yaml')
  );

-- bsl
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('bsl', 'bsl.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.bsl',
    (SELECT id FROM icon_mappings WHERE file_type = 'bsl')
  );

-- mdo
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('mdo', 'mdo.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.mdo',
    (SELECT id FROM icon_mappings WHERE file_type = 'mdo')
  );

-- asm
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('asm', 'asm.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.asm',
    (SELECT id FROM icon_mappings WHERE file_type = 'asm')
  );

-- asm
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('asm', 'asm.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.s',
    (SELECT id FROM icon_mappings WHERE file_type = 'asm')
  );

-- c
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('c', 'c.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.h',
    (SELECT id FROM icon_mappings WHERE file_type = 'c')
  );

-- html
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('html', 'html.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.asax',
    (SELECT id FROM icon_mappings WHERE file_type = 'html')
  );

-- html
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('html', 'html.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.master',
    (SELECT id FROM icon_mappings WHERE file_type = 'html')
  );

-- coldfusion
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('coldfusion', 'coldfusion.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.cfc',
    (SELECT id FROM icon_mappings WHERE file_type = 'coldfusion')
  );

-- coldfusion
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('coldfusion', 'coldfusion.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.cfm',
    (SELECT id FROM icon_mappings WHERE file_type = 'coldfusion')
  );

-- config
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('config', 'config.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.config',
    (SELECT id FROM icon_mappings WHERE file_type = 'config')
  );

-- config
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('config', 'config.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.cfg',
    (SELECT id FROM icon_mappings WHERE file_type = 'config')
  );

-- config
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('config', 'config.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.conf',
    (SELECT id FROM icon_mappings WHERE file_type = 'config')
  );

-- crystal
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('crystal', 'crystal.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.cr',
    (SELECT id FROM icon_mappings WHERE file_type = 'crystal')
  );

-- crystal_embedded
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('crystal_embedded', 'crystal_embedded.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.ecr',
    (SELECT id FROM icon_mappings WHERE file_type = 'crystal_embedded')
  );

-- crystal_embedded
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('crystal_embedded', 'crystal_embedded.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.slang',
    (SELECT id FROM icon_mappings WHERE file_type = 'crystal_embedded')
  );

-- css
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('css', 'css.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.css.map',
    (SELECT id FROM icon_mappings WHERE file_type = 'css')
  );

-- css
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('css', 'css.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.sss',
    (SELECT id FROM icon_mappings WHERE file_type = 'css')
  );

-- csv
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('csv', 'csv.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.csv',
    (SELECT id FROM icon_mappings WHERE file_type = 'csv')
  );

-- xls
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('xls', 'xls.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.xls',
    (SELECT id FROM icon_mappings WHERE file_type = 'xls')
  );

-- xls
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('xls', 'xls.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.xlsx',
    (SELECT id FROM icon_mappings WHERE file_type = 'xls')
  );

-- cake
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('cake', 'cake.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.cake',
    (SELECT id FROM icon_mappings WHERE file_type = 'cake')
  );

-- cake_php
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('cake_php', 'cake_php.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.ctp',
    (SELECT id FROM icon_mappings WHERE file_type = 'cake_php')
  );

-- d
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('d', 'd.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.d',
    (SELECT id FROM icon_mappings WHERE file_type = 'd')
  );

-- word
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('word', 'word.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.doc',
    (SELECT id FROM icon_mappings WHERE file_type = 'word')
  );

-- word
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('word', 'word.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.docx',
    (SELECT id FROM icon_mappings WHERE file_type = 'word')
  );

-- ejs
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('ejs', 'ejs.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.ejs',
    (SELECT id FROM icon_mappings WHERE file_type = 'ejs')
  );

-- elixir_script
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('elixir_script', 'elixir_script.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.exs',
    (SELECT id FROM icon_mappings WHERE file_type = 'elixir_script')
  );

-- favicon
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('favicon', 'favicon.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.ico',
    (SELECT id FROM icon_mappings WHERE file_type = 'favicon')
  );

-- git
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('git', 'git.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.gitignore',
    (SELECT id FROM icon_mappings WHERE file_type = 'git')
  );

-- git
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('git', 'git.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.gitconfig',
    (SELECT id FROM icon_mappings WHERE file_type = 'git')
  );

-- git
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('git', 'git.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.gitkeep',
    (SELECT id FROM icon_mappings WHERE file_type = 'git')
  );

-- git
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('git', 'git.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.gitattributes',
    (SELECT id FROM icon_mappings WHERE file_type = 'git')
  );

-- git
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('git', 'git.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.gitmodules',
    (SELECT id FROM icon_mappings WHERE file_type = 'git')
  );

-- go
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('go', 'go.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.slide',
    (SELECT id FROM icon_mappings WHERE file_type = 'go')
  );

-- go
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('go', 'go.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.article',
    (SELECT id FROM icon_mappings WHERE file_type = 'go')
  );

-- grails
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('grails', 'grails.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.gsp',
    (SELECT id FROM icon_mappings WHERE file_type = 'grails')
  );

-- graphql
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('graphql', 'graphql.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.gql',
    (SELECT id FROM icon_mappings WHERE file_type = 'graphql')
  );

-- graphql
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('graphql', 'graphql.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.graphql',
    (SELECT id FROM icon_mappings WHERE file_type = 'graphql')
  );

-- haskell
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('haskell', 'haskell.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.lhs',
    (SELECT id FROM icon_mappings WHERE file_type = 'haskell')
  );

-- haxe
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('haxe', 'haxe.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.hxs',
    (SELECT id FROM icon_mappings WHERE file_type = 'haxe')
  );

-- haxe
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('haxe', 'haxe.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.hxp',
    (SELECT id FROM icon_mappings WHERE file_type = 'haxe')
  );

-- haxe
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('haxe', 'haxe.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.hxml',
    (SELECT id FROM icon_mappings WHERE file_type = 'haxe')
  );

-- java
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('java', 'java.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.class',
    (SELECT id FROM icon_mappings WHERE file_type = 'java')
  );

-- java
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('java', 'java.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.classpath',
    (SELECT id FROM icon_mappings WHERE file_type = 'java')
  );

-- javascript
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('javascript', 'javascript.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.js.map',
    (SELECT id FROM icon_mappings WHERE file_type = 'javascript')
  );

-- javascript
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('javascript', 'javascript.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.spec.js',
    (SELECT id FROM icon_mappings WHERE file_type = 'javascript')
  );

-- javascript
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('javascript', 'javascript.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.test.js',
    (SELECT id FROM icon_mappings WHERE file_type = 'javascript')
  );

-- javascript
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('javascript', 'javascript.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.es',
    (SELECT id FROM icon_mappings WHERE file_type = 'javascript')
  );

-- javascript
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('javascript', 'javascript.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.es5',
    (SELECT id FROM icon_mappings WHERE file_type = 'javascript')
  );

-- javascript
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('javascript', 'javascript.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.es7',
    (SELECT id FROM icon_mappings WHERE file_type = 'javascript')
  );

-- jinja
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('jinja', 'jinja.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.jinja2',
    (SELECT id FROM icon_mappings WHERE file_type = 'jinja')
  );

-- julia
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('julia', 'julia.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.jl',
    (SELECT id FROM icon_mappings WHERE file_type = 'julia')
  );

-- kotlin
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('kotlin', 'kotlin.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.kts',
    (SELECT id FROM icon_mappings WHERE file_type = 'kotlin')
  );

-- dart
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('dart', 'dart.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.dart',
    (SELECT id FROM icon_mappings WHERE file_type = 'dart')
  );

-- liquid
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('liquid', 'liquid.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.liquid',
    (SELECT id FROM icon_mappings WHERE file_type = 'liquid')
  );

-- livescript
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('livescript', 'livescript.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.ls',
    (SELECT id FROM icon_mappings WHERE file_type = 'livescript')
  );

-- nunjucks
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('nunjucks', 'nunjucks.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.njs',
    (SELECT id FROM icon_mappings WHERE file_type = 'nunjucks')
  );

-- npm
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('npm', 'npm.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.npm-debug.log',
    (SELECT id FROM icon_mappings WHERE file_type = 'npm')
  );

-- npm
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('npm', 'npm.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.npmignore',
    (SELECT id FROM icon_mappings WHERE file_type = 'npm')
  );

-- npm
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('npm', 'npm.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.npmrc',
    (SELECT id FROM icon_mappings WHERE file_type = 'npm')
  );

-- ocaml
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('ocaml', 'ocaml.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.cmx',
    (SELECT id FROM icon_mappings WHERE file_type = 'ocaml')
  );

-- ocaml
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('ocaml', 'ocaml.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.cmxa',
    (SELECT id FROM icon_mappings WHERE file_type = 'ocaml')
  );

-- odata
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('odata', 'odata.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.odata',
    (SELECT id FROM icon_mappings WHERE file_type = 'odata')
  );

-- php
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('php', 'php.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.php.inc',
    (SELECT id FROM icon_mappings WHERE file_type = 'php')
  );

-- pddl
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('pddl', 'pddl.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.pddl',
    (SELECT id FROM icon_mappings WHERE file_type = 'pddl')
  );

-- plan
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('plan', 'plan.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.plan',
    (SELECT id FROM icon_mappings WHERE file_type = 'plan')
  );

-- happenings
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('happenings', 'happenings.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.happenings',
    (SELECT id FROM icon_mappings WHERE file_type = 'happenings')
  );

-- pug
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('pug', 'pug.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.pug',
    (SELECT id FROM icon_mappings WHERE file_type = 'pug')
  );

-- puppet
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('puppet', 'puppet.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.pp',
    (SELECT id FROM icon_mappings WHERE file_type = 'puppet')
  );

-- puppet
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('puppet', 'puppet.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.epp',
    (SELECT id FROM icon_mappings WHERE file_type = 'puppet')
  );

-- react
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('react', 'react.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.spec.jsx',
    (SELECT id FROM icon_mappings WHERE file_type = 'react')
  );

-- react
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('react', 'react.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.test.jsx',
    (SELECT id FROM icon_mappings WHERE file_type = 'react')
  );

-- react
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('react', 'react.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.cjsx',
    (SELECT id FROM icon_mappings WHERE file_type = 'react')
  );

-- react
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('react', 'react.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.spec.tsx',
    (SELECT id FROM icon_mappings WHERE file_type = 'react')
  );

-- react
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('react', 'react.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.test.tsx',
    (SELECT id FROM icon_mappings WHERE file_type = 'react')
  );

-- reasonml
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('reasonml', 'reasonml.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.re',
    (SELECT id FROM icon_mappings WHERE file_type = 'reasonml')
  );

-- html_erb
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('html_erb', 'html_erb.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.erb.html',
    (SELECT id FROM icon_mappings WHERE file_type = 'html_erb')
  );

-- html_erb
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('html_erb', 'html_erb.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.html.erb',
    (SELECT id FROM icon_mappings WHERE file_type = 'html_erb')
  );

-- spring
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('spring', 'spring.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.springbeans',
    (SELECT id FROM icon_mappings WHERE file_type = 'spring')
  );

-- slim
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('slim', 'slim.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.slim',
    (SELECT id FROM icon_mappings WHERE file_type = 'slim')
  );

-- smarty
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('smarty', 'smarty.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.smarty.tpl',
    (SELECT id FROM icon_mappings WHERE file_type = 'smarty')
  );

-- sbt
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('sbt', 'sbt.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.sbt',
    (SELECT id FROM icon_mappings WHERE file_type = 'sbt')
  );

-- scala
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('scala', 'scala.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.scala',
    (SELECT id FROM icon_mappings WHERE file_type = 'scala')
  );

-- ethereum
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('ethereum', 'ethereum.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.sol',
    (SELECT id FROM icon_mappings WHERE file_type = 'ethereum')
  );

-- terraform
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('terraform', 'terraform.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.tf.json',
    (SELECT id FROM icon_mappings WHERE file_type = 'terraform')
  );

-- tex
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('tex', 'tex.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.tex',
    (SELECT id FROM icon_mappings WHERE file_type = 'tex')
  );

-- tex
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('tex', 'tex.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.sty',
    (SELECT id FROM icon_mappings WHERE file_type = 'tex')
  );

-- tex
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('tex', 'tex.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.dtx',
    (SELECT id FROM icon_mappings WHERE file_type = 'tex')
  );

-- tex
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('tex', 'tex.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.ins',
    (SELECT id FROM icon_mappings WHERE file_type = 'tex')
  );

-- config
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('config', 'config.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.toml',
    (SELECT id FROM icon_mappings WHERE file_type = 'config')
  );

-- twig
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('twig', 'twig.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.twig',
    (SELECT id FROM icon_mappings WHERE file_type = 'twig')
  );

-- typescript
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('typescript', 'typescript.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.spec.ts',
    (SELECT id FROM icon_mappings WHERE file_type = 'typescript')
  );

-- typescript
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('typescript', 'typescript.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.test.ts',
    (SELECT id FROM icon_mappings WHERE file_type = 'typescript')
  );

-- vala
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('vala', 'vala.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.vapi',
    (SELECT id FROM icon_mappings WHERE file_type = 'vala')
  );

-- wasm
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('wasm', 'wasm.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.wasm',
    (SELECT id FROM icon_mappings WHERE file_type = 'wasm')
  );

-- wat
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('wat', 'wat.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.wat',
    (SELECT id FROM icon_mappings WHERE file_type = 'wat')
  );

-- zip
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('zip', 'zip.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.jar',
    (SELECT id FROM icon_mappings WHERE file_type = 'zip')
  );

-- zip
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('zip', 'zip.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.zip',
    (SELECT id FROM icon_mappings WHERE file_type = 'zip')
  );

-- wgt
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('wgt', 'wgt.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.wgt',
    (SELECT id FROM icon_mappings WHERE file_type = 'wgt')
  );

-- illustrator
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('illustrator', 'illustrator.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.ai',
    (SELECT id FROM icon_mappings WHERE file_type = 'illustrator')
  );

-- photoshop
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('photoshop', 'photoshop.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.psd',
    (SELECT id FROM icon_mappings WHERE file_type = 'photoshop')
  );

-- pdf
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('pdf', 'pdf.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.pdf',
    (SELECT id FROM icon_mappings WHERE file_type = 'pdf')
  );

-- font
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('font', 'font.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.eot',
    (SELECT id FROM icon_mappings WHERE file_type = 'font')
  );

-- font
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('font', 'font.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.ttf',
    (SELECT id FROM icon_mappings WHERE file_type = 'font')
  );

-- font
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('font', 'font.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.woff',
    (SELECT id FROM icon_mappings WHERE file_type = 'font')
  );

-- font
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('font', 'font.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.woff2',
    (SELECT id FROM icon_mappings WHERE file_type = 'font')
  );

-- image
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('image', 'image.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.gif',
    (SELECT id FROM icon_mappings WHERE file_type = 'image')
  );

-- image
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('image', 'image.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.jpg',
    (SELECT id FROM icon_mappings WHERE file_type = 'image')
  );

-- image
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('image', 'image.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.jpeg',
    (SELECT id FROM icon_mappings WHERE file_type = 'image')
  );

-- image
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('image', 'image.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.png',
    (SELECT id FROM icon_mappings WHERE file_type = 'image')
  );

-- image
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('image', 'image.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.pxm',
    (SELECT id FROM icon_mappings WHERE file_type = 'image')
  );

-- svg
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('svg', 'svg.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.svg',
    (SELECT id FROM icon_mappings WHERE file_type = 'svg')
  );

-- image
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('image', 'image.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.svgx',
    (SELECT id FROM icon_mappings WHERE file_type = 'image')
  );

-- sublime
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('sublime', 'sublime.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.sublime-project',
    (SELECT id FROM icon_mappings WHERE file_type = 'sublime')
  );

-- sublime
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('sublime', 'sublime.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.sublime-workspace',
    (SELECT id FROM icon_mappings WHERE file_type = 'sublime')
  );

-- salesforce
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('salesforce', 'salesforce.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.component',
    (SELECT id FROM icon_mappings WHERE file_type = 'salesforce')
  );

-- salesforce
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('salesforce', 'salesforce.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.cls',
    (SELECT id FROM icon_mappings WHERE file_type = 'salesforce')
  );

-- shell
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('shell', 'shell.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.fish',
    (SELECT id FROM icon_mappings WHERE file_type = 'shell')
  );

-- video
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('video', 'video.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.mov',
    (SELECT id FROM icon_mappings WHERE file_type = 'video')
  );

-- video
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('video', 'video.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.ogv',
    (SELECT id FROM icon_mappings WHERE file_type = 'video')
  );

-- video
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('video', 'video.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.webm',
    (SELECT id FROM icon_mappings WHERE file_type = 'video')
  );

-- video
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('video', 'video.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.avi',
    (SELECT id FROM icon_mappings WHERE file_type = 'video')
  );

-- video
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('video', 'video.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.mpg',
    (SELECT id FROM icon_mappings WHERE file_type = 'video')
  );

-- video
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('video', 'video.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.mp4',
    (SELECT id FROM icon_mappings WHERE file_type = 'video')
  );

-- audio
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('audio', 'audio.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.mp3',
    (SELECT id FROM icon_mappings WHERE file_type = 'audio')
  );

-- audio
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('audio', 'audio.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.ogg',
    (SELECT id FROM icon_mappings WHERE file_type = 'audio')
  );

-- audio
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('audio', 'audio.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.wav',
    (SELECT id FROM icon_mappings WHERE file_type = 'audio')
  );

-- audio
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('audio', 'audio.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.flac',
    (SELECT id FROM icon_mappings WHERE file_type = 'audio')
  );

-- svg
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('svg', 'svg.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.3ds',
    (SELECT id FROM icon_mappings WHERE file_type = 'svg')
  );

-- svg
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('svg', 'svg.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.3dm',
    (SELECT id FROM icon_mappings WHERE file_type = 'svg')
  );

-- svg
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('svg', 'svg.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.stl',
    (SELECT id FROM icon_mappings WHERE file_type = 'svg')
  );

-- svg
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('svg', 'svg.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.obj',
    (SELECT id FROM icon_mappings WHERE file_type = 'svg')
  );

-- svg
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('svg', 'svg.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.dae',
    (SELECT id FROM icon_mappings WHERE file_type = 'svg')
  );

-- babel
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('babel', 'babel.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.babelrc',
    (SELECT id FROM icon_mappings WHERE file_type = 'babel')
  );

-- docker
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('docker', 'docker.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.dockerignore',
    (SELECT id FROM icon_mappings WHERE file_type = 'docker')
  );

-- code-climate
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('code-climate', 'code-climate.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.codeclimate.yml',
    (SELECT id FROM icon_mappings WHERE file_type = 'code-climate')
  );

-- eslint
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('eslint', 'eslint.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.eslintrc',
    (SELECT id FROM icon_mappings WHERE file_type = 'eslint')
  );

-- eslint
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('eslint', 'eslint.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.eslintrc.js',
    (SELECT id FROM icon_mappings WHERE file_type = 'eslint')
  );

-- eslint
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('eslint', 'eslint.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.eslintrc.yaml',
    (SELECT id FROM icon_mappings WHERE file_type = 'eslint')
  );

-- eslint
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('eslint', 'eslint.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.eslintrc.yml',
    (SELECT id FROM icon_mappings WHERE file_type = 'eslint')
  );

-- eslint
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('eslint', 'eslint.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.eslintrc.json',
    (SELECT id FROM icon_mappings WHERE file_type = 'eslint')
  );

-- eslint
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('eslint', 'eslint.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.eslintignore',
    (SELECT id FROM icon_mappings WHERE file_type = 'eslint')
  );

-- firebase
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('firebase', 'firebase.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.firebaserc',
    (SELECT id FROM icon_mappings WHERE file_type = 'firebase')
  );

-- stylelint
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('stylelint', 'stylelint.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.stylelintrc',
    (SELECT id FROM icon_mappings WHERE file_type = 'stylelint')
  );

-- stylelint
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('stylelint', 'stylelint.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.stylelintrc.json',
    (SELECT id FROM icon_mappings WHERE file_type = 'stylelint')
  );

-- stylelint
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('stylelint', 'stylelint.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.stylelintrc.yaml',
    (SELECT id FROM icon_mappings WHERE file_type = 'stylelint')
  );

-- stylelint
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('stylelint', 'stylelint.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.stylelintrc.yml',
    (SELECT id FROM icon_mappings WHERE file_type = 'stylelint')
  );

-- stylelint
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('stylelint', 'stylelint.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.stylelintrc.js',
    (SELECT id FROM icon_mappings WHERE file_type = 'stylelint')
  );

-- stylelint
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('stylelint', 'stylelint.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.stylelintignore',
    (SELECT id FROM icon_mappings WHERE file_type = 'stylelint')
  );

-- config
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('config', 'config.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.direnv',
    (SELECT id FROM icon_mappings WHERE file_type = 'config')
  );

-- config
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('config', 'config.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.env',
    (SELECT id FROM icon_mappings WHERE file_type = 'config')
  );

-- config
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('config', 'config.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.static',
    (SELECT id FROM icon_mappings WHERE file_type = 'config')
  );

-- config
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('config', 'config.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.editorconfig',
    (SELECT id FROM icon_mappings WHERE file_type = 'config')
  );

-- config
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('config', 'config.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.slugignore',
    (SELECT id FROM icon_mappings WHERE file_type = 'config')
  );

-- clock
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('clock', 'clock.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.tmp',
    (SELECT id FROM icon_mappings WHERE file_type = 'clock')
  );

-- config
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('config', 'config.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.htaccess',
    (SELECT id FROM icon_mappings WHERE file_type = 'config')
  );

-- lock
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('lock', 'lock.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.key',
    (SELECT id FROM icon_mappings WHERE file_type = 'lock')
  );

-- lock
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('lock', 'lock.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.cert',
    (SELECT id FROM icon_mappings WHERE file_type = 'lock')
  );

-- ignored
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('ignored', 'ignored.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_extensions (extension, icon_mapping_id)
  VALUES (
    '.ds_store',
    (SELECT id FROM icon_mappings WHERE file_type = 'ignored')
  );

-- hex
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('hex', 'hex.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'mix',
    (SELECT id FROM icon_mappings WHERE file_type = 'hex')
  );

-- karma
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('karma', 'karma.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'karma.conf.js',
    (SELECT id FROM icon_mappings WHERE file_type = 'karma')
  );

-- karma
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('karma', 'karma.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'karma.conf.coffee',
    (SELECT id FROM icon_mappings WHERE file_type = 'karma')
  );

-- info
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('info', 'info.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'readme.md',
    (SELECT id FROM icon_mappings WHERE file_type = 'info')
  );

-- clock
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('clock', 'clock.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'changelog.md',
    (SELECT id FROM icon_mappings WHERE file_type = 'clock')
  );

-- clock
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('clock', 'clock.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'changelog',
    (SELECT id FROM icon_mappings WHERE file_type = 'clock')
  );

-- clock
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('clock', 'clock.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'changes.md',
    (SELECT id FROM icon_mappings WHERE file_type = 'clock')
  );

-- clock
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('clock', 'clock.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'version.md',
    (SELECT id FROM icon_mappings WHERE file_type = 'clock')
  );

-- clock
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('clock', 'clock.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'version',
    (SELECT id FROM icon_mappings WHERE file_type = 'clock')
  );

-- maven
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('maven', 'maven.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'mvnw',
    (SELECT id FROM icon_mappings WHERE file_type = 'maven')
  );

-- tsconfig
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('tsconfig', 'tsconfig.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'tsconfig.json',
    (SELECT id FROM icon_mappings WHERE file_type = 'tsconfig')
  );

-- json
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('json', 'json.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'swagger.json',
    (SELECT id FROM icon_mappings WHERE file_type = 'json')
  );

-- json
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('json', 'json.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'swagger.yml',
    (SELECT id FROM icon_mappings WHERE file_type = 'json')
  );

-- json
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('json', 'json.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'swagger.yaml',
    (SELECT id FROM icon_mappings WHERE file_type = 'json')
  );

-- config
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('config', 'config.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'mime.types',
    (SELECT id FROM icon_mappings WHERE file_type = 'config')
  );

-- bower
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('bower', 'bower.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'bower.json',
    (SELECT id FROM icon_mappings WHERE file_type = 'bower')
  );

-- docker
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('docker', 'docker.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'docker-healthcheck',
    (SELECT id FROM icon_mappings WHERE file_type = 'docker')
  );

-- docker
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('docker', 'docker.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'docker-compose.yml',
    (SELECT id FROM icon_mappings WHERE file_type = 'docker')
  );

-- docker
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('docker', 'docker.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'docker-compose.yaml',
    (SELECT id FROM icon_mappings WHERE file_type = 'docker')
  );

-- docker
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('docker', 'docker.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'docker-compose.override.yml',
    (SELECT id FROM icon_mappings WHERE file_type = 'docker')
  );

-- docker
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('docker', 'docker.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'docker-compose.override.yaml',
    (SELECT id FROM icon_mappings WHERE file_type = 'docker')
  );

-- firebase
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('firebase', 'firebase.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'firebase.json',
    (SELECT id FROM icon_mappings WHERE file_type = 'firebase')
  );

-- firefox
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('firefox', 'firefox.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'geckodriver',
    (SELECT id FROM icon_mappings WHERE file_type = 'firefox')
  );

-- grunt
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('grunt', 'grunt.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'gruntfile.js',
    (SELECT id FROM icon_mappings WHERE file_type = 'grunt')
  );

-- grunt
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('grunt', 'grunt.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'gruntfile.babel.js',
    (SELECT id FROM icon_mappings WHERE file_type = 'grunt')
  );

-- grunt
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('grunt', 'grunt.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'gruntfile.coffee',
    (SELECT id FROM icon_mappings WHERE file_type = 'grunt')
  );

-- gulp
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('gulp', 'gulp.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'gulpfile',
    (SELECT id FROM icon_mappings WHERE file_type = 'gulp')
  );

-- ionic
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('ionic', 'ionic.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'ionic.config.json',
    (SELECT id FROM icon_mappings WHERE file_type = 'ionic')
  );

-- ionic
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('ionic', 'ionic.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'ionic.project',
    (SELECT id FROM icon_mappings WHERE file_type = 'ionic')
  );

-- platformio
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('platformio', 'platformio.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'platformio.ini',
    (SELECT id FROM icon_mappings WHERE file_type = 'platformio')
  );

-- rollup
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('rollup', 'rollup.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'rollup.config.js',
    (SELECT id FROM icon_mappings WHERE file_type = 'rollup')
  );

-- sass
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('sass', 'sass.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'sass-lint.yml',
    (SELECT id FROM icon_mappings WHERE file_type = 'sass')
  );

-- stylelint
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('stylelint', 'stylelint.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'stylelint.config.js',
    (SELECT id FROM icon_mappings WHERE file_type = 'stylelint')
  );

-- yarn
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('yarn', 'yarn.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'yarn.clean',
    (SELECT id FROM icon_mappings WHERE file_type = 'yarn')
  );

-- yarn
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('yarn', 'yarn.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'yarn.lock',
    (SELECT id FROM icon_mappings WHERE file_type = 'yarn')
  );

-- webpack
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('webpack', 'webpack.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'webpack.config.js',
    (SELECT id FROM icon_mappings WHERE file_type = 'webpack')
  );

-- webpack
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('webpack', 'webpack.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'webpack.config.build.js',
    (SELECT id FROM icon_mappings WHERE file_type = 'webpack')
  );

-- webpack
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('webpack', 'webpack.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'webpack.common.js',
    (SELECT id FROM icon_mappings WHERE file_type = 'webpack')
  );

-- webpack
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('webpack', 'webpack.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'webpack.dev.js',
    (SELECT id FROM icon_mappings WHERE file_type = 'webpack')
  );

-- webpack
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('webpack', 'webpack.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'webpack.prod.js',
    (SELECT id FROM icon_mappings WHERE file_type = 'webpack')
  );

-- license
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('license', 'license.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'license',
    (SELECT id FROM icon_mappings WHERE file_type = 'license')
  );

-- license
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('license', 'license.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'licence',
    (SELECT id FROM icon_mappings WHERE file_type = 'license')
  );

-- license
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('license', 'license.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'copying',
    (SELECT id FROM icon_mappings WHERE file_type = 'license')
  );

-- license
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('license', 'license.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'compiling',
    (SELECT id FROM icon_mappings WHERE file_type = 'license')
  );

-- license
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('license', 'license.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'contributing',
    (SELECT id FROM icon_mappings WHERE file_type = 'license')
  );

-- makefile
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('makefile', 'makefile.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'qmakefile',
    (SELECT id FROM icon_mappings WHERE file_type = 'makefile')
  );

-- makefile
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('makefile', 'makefile.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'omakefile',
    (SELECT id FROM icon_mappings WHERE file_type = 'makefile')
  );

-- makefile
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('makefile', 'makefile.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'cmakelists.txt',
    (SELECT id FROM icon_mappings WHERE file_type = 'makefile')
  );

-- heroku
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('heroku', 'heroku.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'procfile',
    (SELECT id FROM icon_mappings WHERE file_type = 'heroku')
  );

-- npm_ignored
INSERT INTO icon_mappings (file_type, image_file)
  VALUES ('npm_ignored', 'npm_ignored.svg')
  ON CONFLICT DO NOTHING;
INSERT INTO file_names (name, icon_mapping_id)
  VALUES (
    'npm-debug.log',
    (SELECT id FROM icon_mappings WHERE file_type = 'npm_ignored')
  );
