# MDL
```ruby

#main.mdl
App {
  body: FlexLayout {
    direction: column
    alignItens: center
    children: [
      Text {
        id: inc
        data: { val = 0 }
        text: "Value is ${val}"
      },
      SizedBox { height: 30 },
      MyComp {
        extern_name: "My component"
      },
      SizedBox { height: 30 }, 
      Button {
        text: "Increment"
        onClick: (ev) {
          val++
        }
      },
      Button {
        text: "Add Item"
        onClick: (ev: mouseEvent) {
          
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
  props: { extern_name: string }
  define: {
    Column {
      children: [
        Text {
          id: "items_count"
          text: "Name from outside ${props.extern_name}. Count: $[get_itens_count()}"
        },
        List {
          onUpdate: (ev: ctx) => {
            @items_count.update()
          }
          data: {items: number[]}
          items: get_items()
        }
      ]
    }
  }
}
```