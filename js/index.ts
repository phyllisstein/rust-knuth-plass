void (async () => {
    const rust = await import('../rust/pkg/index.js')
    const txt = rust.break_lines('Hello, world')
    console.log({ txt })
})()
