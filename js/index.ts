import {IMyInterface, tpipe} from "./typedefs.ts"

import('../rust/pkg').then((module) => {
    module.greet('World')
    console.log(pipe())
    
    const someObjects: IMyInterface[] = [
        { myProperty: 'a', myFunction: () => console.log('a') },
        { myProperty: 'b', myFunction: () => console.log('b') },
    ]
    let obj = someObjects[0]
})
