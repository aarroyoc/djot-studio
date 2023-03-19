const source = document.getElementById("source");
const output = document.getElementById("output");

const open = document.getElementById("open");
const saveDjot = document.getElementById("save-djot");
const saveHtml = document.getElementById("save-html");

function render() {
    fetch("/", {
	method: "POST",
	body: source.value
    })
    .then(resp => resp.text())
    .then(text => output.innerHTML = text);
}

source.oninput = render;

open.oninput = () => {
    if(open.files.length == 1) {
	open.files[0].text().then(t => {
	    source.value = t;
	    render();
	});
    }
};

saveDjot.onclick = () => {
    const blob = new Blob([source.value], {
	type: "text/plain"
    });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.download = true;
    a.href = url;
    a.click();
};

saveHtml.onclick = () => {
    const blob = new Blob([output.innerHTML], {
	type: "text/html"
    });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.download = true;
    a.href = url;
    a.click();
};
