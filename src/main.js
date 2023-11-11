let developmentMode = true;

function pollDevelopmentMode() {
	return developmentMode;
}

function panic(title, message) {
	document.getElementById("errortitle").textContent = title;
	document.getElementById("errormessage").textContent = message;

	const panic = document.getElementById("panic");
	panic.classList.add("onTop");
	showElementById(panic);
	panic.style.opacity = 1;
}

function hideElementById(element) {
	element.classList.add("fadeout");
	setTimeout(function() {
		element.classList.add("hidden");
		element.classList.remove("fadeout");
	}, 300);
}
function showElementById(element) {
	element.classList.remove("hidden");
	element.classList.add("fadein");
	setTimeout(function() {
		element.classList.remove("fadein");
	}, 300);
}

if (!developmentMode) {
	setTimeout(function() {
		// microsoft webview sucks; it displays white for like 3 seconds before actually doing something
		// so we just ease it in
		document.body.style.backgroundColor = "#1b1b1b";

		var opacity = 0;
		var fadein = setInterval(function () {
			if (opacity > 1) {
				clearInterval(fadein);
			}
			document.body.style.opacity = opacity;
			opacity += 0.02;
		}, 10);
	}, 300);
} else {
	document.body.style.backgroundColor = "#1b1b1b";
	document.body.style.opacity = 1;
}

window.addEventListener('contextmenu', e => {
	e.preventDefault();
});
