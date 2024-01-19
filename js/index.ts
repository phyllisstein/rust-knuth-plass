void (async () => {
    const rust = await import('../rust/pkg/index.js')
    const txt = rust.break_lines()
    console.log({ txt })
})()
