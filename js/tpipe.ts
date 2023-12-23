import {IMyInterface} from "./typedefs.js";

export function tpipe () {
    console.log('pipe')

    const someObjects: IMyInterface[] = [
        { myProperty: 'a', myFunction: () => console.log('a') },
        { myProperty: 'b', myFunction: () => console.log('b') },
    ]

    let ret = Object.values(someObjects)
        .map(p => `property: ${p.myProperty}`)
        |> `values: ${# }`

    let et = do {
        let x = 1
        let y = 2
        x + y
    }

    return (et as unknown as number)
}
