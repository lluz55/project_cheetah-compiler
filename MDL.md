# MDL

## Language objectives
- Use components for compose user interface
- Acessible from native code using bridge
- Like typescript when using functions
- Primarly functional
- Data oriented
- Reactive

```ruby
#main.mdl

import { MyComp } from "./mycomp"

App {
  body: Column {
    justifyContent: center
    children: [
      Text {
        id: "inc"
        data: { val = 0 }
        text: "Value is ${val}"
      },
      SizedBox { height: 30 },
      MyComp {
        extern_name: "My component"
        onExported_event: (ev: mouseEvent) => {
          debug("I'm event from MyComp")
        }
      },
      SizedBox { height: 30 }, 
      Button {
        text: "Increment"
        onClick: (ev) {
          @inc.val++
        }
      }
    ]
  }
}
```

#MyComp.mdl
```typescript
function get_itens_count(items: number[]): number {
  return items.length
}

function get_items(items: number[]): Component[] {
  return items.map((idx, item) => {
    if (idx % 0) {
      return ListItem {
        child: Text {
          text: "Item even #${index}"
        }
      }
    } else {
      return ListItem {
        child: Text {
          text: "Item odd #${index}"
        }
      }
    }
  })
}
```

```ruby
export MyComp {
  props: { extern_name: string, onExported_event: Function(ev: mouseEvent) }
  define: {
    Column {
      children: [
        Text {
          id: "items_count"
          text: "Name from outside ${props.extern_name}. Count: ${get_itens_count()}"
        },
        List {
          onUpdate: (ev: context) => {
            @items_count.update()
          }
          data: {items: number[]}
          items: get_items()
        },
        Button {
          text: "click to fire event"
          onClick: (ev: mouseEvent) => {
            exported_event(ev)
          }
        }
      ]
    }
  }
}
```