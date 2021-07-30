# Kuri
## A multipurpose boilerplate generator

Usage: kuri generate \<blueprint> \<module name>

Configuration
-------------
Kuri is configured using a `kuri.toml` file. Kuri will look for a kuri.toml file in the project's root directory.<br>

example `kuri.toml`
```toml
[project] #The general configuration for the project - required
project_name="test" #required
src_dir="out" #not required
blueprint_dir="blueprints" #not required
version="v0.0.1" #not required
license="LICENSE" #not required
repo="https://github.com/mx-mw/kuri" #not required

[template] #the project template (e.g. node, ember, CRA, cargo, etc) - not required
language="rust" 
variant="cargo"

[meta] #kuri metadata... for now only consists of a version - required
kuri_version="0.0.1" #required

[flags] #allows you to replace the default flags with your own - not required
module_name_rep="[[ModuleName]]"
license_rep="[[License]]"
version_rep="[[Version]]

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
            <p>%!%ModuleName%!%</p>
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
            <p>MyComponent</p>
        </div>
    }
}
```

