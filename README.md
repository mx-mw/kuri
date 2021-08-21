<img src="https://www.code-inspector.com/project/25868/status/svg"></img>
# Kuri
## A multipurpose blueprint generator
Kuri is a simple blueprinting tool to speed up your development process.

Usage: kuri generate \<blueprint> \<module name>

Table of contents:
------------------
1. [Boilerplates](#Boilerplates)
2. [Configuration](#Configuration)
3. [Examples](#Examples)

Boilerplates
------------

### Flags:
`%!%ModuleName%!%` = the name of the module you are generating (passed as the 3rd positional argument)
`%!%License%!%`    = the project's license (the *path* to the file is specified in kuri.toml, not the whole fucking GPLv3)
`%!%Version%!%`    = the current version of the project (specified in kuri.toml)

### Things to note:
- Flags are case insensitive
- You cannot escape a flag if there is a naming conflict - they can be reconfigured in kuri.toml


Configuration
-------------
Kuri is configured using a `kuri.toml` file. Kuri will look for a kuri.toml file in the project's root directory.<br>

example `kuri.toml`
```toml
[project] # The general configuration for the project - required
project_name="test" # required
src_dir="out" # not required
blueprint_dir="blueprints" # not required
version="v0.0.1" # not required
license="LICENSE" # not required
repo="https://github.com/mx-mw/kuri" # not required

[template] # the project template (e.g. node, ember, CRA, cargo, etc)... not used for now - not required
language="rust" 
variant="cargo"

[meta] # kuri metadata... for now only consists of a version... not used for now - required
kuri_version="0.0.1" # required

[flags] # allows you to replace the default flags with your own - not required
module_name_rep="[[ModuleName]]"
custom_flags=[ # kuri allows you to create custom flags. the | and everything before it is ignored
    {name="ArgTest", source="arg", replace_with="1"}, # create a custom flag that is replaced by a positional argument (counting from 1). This example would replace %!%ArgTest%!% with the first positional argument passed to kuri
    {name="StringTest", source="str", replace_with="wonderful string"}, # create a flag that is replaced by a string. This would replace %!%StringTest%!% with "wonderful string"
    {name="FileTest", source="file", replace_with="LICENSE"}, # create a flag that is replaced by the contents of a file. This example would replace %!%FileTest%!% with the contents of the LICENSE file
    {name="ArgFileTest", source="argfile", replace_with="2"} # create a flag that is replaced by the contents of a file specified with a positional argument. This example would replace %!%ArgFileTest%!% with the contents of a file specified with the second positional argument
    ]

```

Example Usage
-------------
### E.G. 1: Generate C++ header/code file
`blueprints/class.h.kbp`
```kbp
#pragma once

class %!%ModuleName%!% 
{
public:
    %!%ModuleName%!%();
    ~%!%ModuleName%!%();
private:

protected:

}
```

`blueprints/class.cpp.kbp`
```kbp
#pragma once
#include "%!%ModuleName%!%.h"

%!%ModuleName%!%::%!%ModuleName%!%() 
{

}

%!%ModuleName%!%::~%!%ModuleName%!%() 
{

}
```
`$ kuri g class MyClass` <br><br>
Output: <br><br>
`src/MyClass.h`
```h
#pragma once

class MyClass
{
public:
    MyClass();
    ~MyClass();
private:

protected:

}
```

`src/MyClass.cpp`
```cpp
#pragma once
#include "MyClass.h"

MyClass::MyClass() 
{

}

MyClass::~MyClass() 
{

}
```

### E.G. 2: Generate react component
`blueprints/component.tsx.kbp`
```kbp
'use-strict'
import React from 'react';

export default class %!%ModuleName%!% extends React.Component {
    render() {
        return <>
            <p>%!%ModuleName%!% %!%Version%!%</p>
        </>
    }
}
```

`$ kuri g class MyComponent` <br><br>
Output: <br><br>
`src/MyComponent.h`
```tsx
'use-strict'
import React from 'react';

export default class MyComponent extends React.Component {
    render() {
        return <div className="MyComponent">
            <p>MyComponent v1.5.1</p>
        </div>
    }
}
```

### E.G. 3: Generate HTML and CSS pair
`blueprints/css/page.css.kbp`
```kbp
@import url('https://fonts.googleapis.com/css2?family=Ubuntu:wght@300;400;500;700&display=swap');

#%!%ModuleName%!% {
    font-family: 'Ubuntu', sans-serif;
    font-size:   16px;
}
```

`blueprints/html/page.html.kbp`
```kbp
<!DOCTYPE html>
<html lang="en">
<head>
    <link rel="stylesheet" href="../css/%!%ModuleName%!%.css">
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>%!%ModuleName%!%</title>
</head>
<div id="%!%ModuleName%!%">
    <p>%!%ModuleName%!% %!%Version%!%</p>
</div>
</html>
```

`$ kuri g page Homepage` <br><br>
Output: <br><br>
`src/css/Homepage.css`
```css
@import url('https://fonts.googleapis.com/css2?family=Ubuntu:wght@300;400;500;700&display=swap');

#Homepage {
    font-family: 'Ubuntu', sans-serif;
    font-size:   16px;
}
```

`src/html/Homepage.html`
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <link rel="stylesheet" href="../css/Homepage.css">
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Homepage</title>
</head>
<div id="Homepage">
    <p>Homepage v0.0.1</p>
</div>
</html>
```