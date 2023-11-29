let develop = false;

// run this file with `async` boolean attribute
// this file contains code that should be used everywhere here.
window.addEventListener('contextmenu', e => {
	e.preventDefault();
});
window.addEventListener('keydown', e=>{if(!develop)e.preventDefault()});
