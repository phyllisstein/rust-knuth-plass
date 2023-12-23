export function pipe () {
    console.log('pipe')

    const someObjects = [
        { myProperty: 'a', myFunction: () => console.log('a') },
        { myProperty: 'b', myFunction: () => console.log('b') },
    ]

    let ret = Object.values(someObjects)
        .map(p => `property: ${p.myProperty}`)
        |> `values: ${#}`

    let et = do {
        let x = 1
        let y = 2
        x + y
    }

    let obj = someObjects[0]

    return ret
}
