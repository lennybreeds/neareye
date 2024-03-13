chrome.runtime.onMessage.addListener(
    function (request, sender, sendResponse) {
        // console.log('received a request')
        if (request.message === "show-banner") {
            // console.log("modify banner received")
            // document.body.style.backgroundColor = "yellow";
            // Create a new div element for the banner
            const banner = document.createElement('div');

            // Set some styles to make the banner visible and styled at the top
            banner.style.position = 'absolute';
            banner.style.height = '36px'
            banner.style.width = '100%';
            banner.style.backgroundColor = '#80DB71'; // Green background
            banner.style.color = 'black';
            banner.style.fontSize = '12'
            banner.style.textAlign = 'center';
            banner.style.display = 'flex';
            banner.style.flexDirection = 'row';
            banner.style.alignItems = 'center';
            banner.style.justifyContent = 'center';
            banner.style.zIndex = '1000'; // Ensure it's on top of other content
            banner.style.top = '0';
            banner.style.left = '0';
            banner.style.fontWeight = '600'
            banner.textContent = 'SolEye: Passed all checks'; // The text to display in the banner

            // Insert the banner at the top of the body element
            document.body.prepend(banner);

            // Adjust body padding so the banner doesn't overlap content
            document.body.style.paddingTop = `${banner.offsetHeight}px`;
        }
    }
);

function start() {
    alert("started");
}

chrome.runtime.sendMessage({ site: "new" }, function (response) {
    // console.log('sent new site notif')

    const banner = document.createElement('div');

    // Set some styles to make the banner visible and styled at the top
    banner.style.position = 'absolute';
    banner.style.height = '36px';
    banner.style.width = '100%';
    banner.style.backgroundColor = '#989898'; // Green background
    banner.style.color = 'black';
    banner.style.fontSize = '12';
    banner.style.textAlign = 'center';
    banner.style.display = 'flex';
    banner.style.flexDirection = 'row';
    banner.style.alignItems = 'center';
    banner.style.justifyContent = 'center';
    banner.style.zIndex = '1000'; // Ensure it's on top of other content
    banner.style.top = '0';
    banner.style.left = '0';
    banner.style.fontWeight = '600';
    banner.textContent = 'SolEye: Analyzing site'; // The text to display in the banner

    // Insert the banner at the top of the body element
    document.body.prepend(banner);

    let current_url;
    current_url = window.location.href;
    current_content = document.body.innerText;

    // console.log(current_url)

    chrome.storage.sync.set({ 'current_url': current_url }, function () {
    });

    // console.log('sending safety check')
    chrome.runtime.sendMessage({ req: 'safety', url: current_url, content: current_content, code: '' }, response => {
        // console.log('response: ', response)

        chrome.storage.sync.set({ 'reply': response }, function () {
        });

        if (response.probability <= 0.4) {
            banner.textContent = 'SolEye: Website safe'; // The text to display in the banner
            banner.style.backgroundColor = '#80DB71'; // Green background
        }
        else {
            banner.textContent = 'SolEye: Website unsafe'; // The text to display in the banner
            banner.style.backgroundColor = '#DB7171'; // Green background
        }
    })

    // http://127.0.0.1:8000
    // Define an async function to use await
    document.body.style.paddingTop = `${banner.offsetHeight}px`;
    // // console.log(response.res);
});