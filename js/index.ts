import './index.css'

void (async () => {
    const rust = await import('../rust/pkg/index.js')
    const txt = rust.break_lines('Five Reasons Drinking Milk On the Toilet Is Kind of a Game-Changer')
    document.querySelector('.message').innerHTML = txt
})()
