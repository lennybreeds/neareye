from openai import OpenAI
from fastapi import FastAPI, Response
from pydantic import BaseModel
from fastapi.responses import JSONResponse
from fastapi.encoders import jsonable_encoder
import requests
from datetime import datetime
import json
import anthropic
import subprocess
app = FastAPI()
class Data(BaseModel):
    url: str
    content: str
    code: str
    name: str

async def get_domain_age(domain_name, api_key):
    Url = f"https://www.whoisxmlapi.com/whoisserver/WhoisService?apiKey={api_key}&domainName={domain_name}&outputFormat=JSON"
    response = requests.get(Url)
    domain_data = response.json()
    print(domain_data)
    creation_date_str = domain_data['WhoisRecord']['registryData']['createdDateNormalized']
    creation_date = datetime.strptime(creation_date_str, "%Y-%m-%d %H:%M:%S %Z")
    current_date = datetime.utcnow()
    domain_age = current_date - creation_date
    
    return domain_age.days

@app.post("/check-safety/")
async def check_safety(data: Data):
    max_length = 10000

    prompt = f"""Is the URL {data.url} with site content {data.content[:max_length]}, domain age in days, {await get_domain_age(data.name, 'at_4WCiJ8ndK1naj0xJGzI0a2WPmlwZz')}, javascript code {data.code[:max_length]} a scam website,
                Then give clear and not obvious or verbose reasons as to why the website may be a scam with as few words as possible describing each reason.
                Only give reasons for the website being a scam. Use your common sense, if the URL is fine then the rest of the stuff is probably fine.
                Make sure you seperate the reasons into four categories, site content reasons, javascript code reasons, URL reasons, and domain age reasons. 
                Only include reasons that suggest scamming if your reason is "None" omit the entire category. A scam domain would have a young domain age i.e less than 180 days old. If the domain age is above
                180 days old it is not a scam and you should return domain age reasons with None. Then make a final category called synopsis which gives a brief synopsis of why you have chosen the specific probabiity ratings. Do not include any colons in your response or any punctuation. Evaluate these
                answers and answer only with a probability rating from 0-1. 0 being a safe site, 1 being 100% a scam.
                A model response would be in A model response would be in JSON format with the keys "probability", "site content reasons", "javascript code reasons", "URL reasons", "domain age reasons", "synopsis". The following is a model response:
                                          "probability": 0.95,
                                            "site content reasons": "Inappropriate content|Misleading information",
                                            "javascript code reasons": "None",
                                            "URL reasons": "None",
                                            "domain age reasons": "None",
                                            "synopsis": Probability of 0.95 was chosen due to...
                This part is incredibly important do not forget to do this your response should be in JSON format with probability and each reason split by |. This part is also very important write this all in one line 
                as I'll convert your response javascript so I don't want any /n and replace all / with ' to appear in the response anywhere."""


    client = anthropic.Anthropic(
        
        api_key="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",
    )

    message = client.messages.create(
        model="claude-3-opus-20240229",
        max_tokens=1000,
        temperature=0.0,
        system="You will be given specific data from websites and you will have to analyse this data and tell me whether the website is safe or a scam.",
        messages=[
            {"role": "user", "content": prompt}
        ]
    )
    
    clauderesponse = message.content

    domainresponse = await get_domain_age(data.name, 'at_4WCiJ8ndK1naj0xJGzI0a2WPmlwZz')

    response = json.loads(clauderesponse[0].text)
    response['domain_age'] = domainresponse
    addtosolana(response)
    # return response
    return response


def addtosolana(response):
    arguments = {
    "url": response['url'],
    "url_reasons": response['url reasons'],
    "probability": response['probability'],
    "whitelist": response['whitelist'],
    "domain_age_reasons": response['domain age reasons'],
    "javascript_code_reasons": response['javascript code reasons'],
    "site_content_reasons": response['site content reasons'],
    }
    arguments_json = json.dumps(arguments)

    command = ["node", "addtosolana.mjs", arguments_json]

    # Execute the command
    process = subprocess.run(command, capture_output=True, text=True)

    # Check if the process was successful
    if process.returncode == 0:
        print("Success!")
        print("Output:", process.stdout)
    else:
        print("Error!")
        print("Output:", process.stderr)

    return process.stdout