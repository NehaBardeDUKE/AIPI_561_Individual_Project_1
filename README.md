# Sentiment Analysis service

With this project i tried to do a POC for the zero shot sentiment analysis (inference) using the libraries avaialble in Rust. I made use of 2 libraries each having their own way of ranking a sentence having a certain sentiment.
The tests were done using a cli tool built using clap. This provided a quick and easy way of prototyping and trying out different things quickly. Here only 1 sentence can be passed at runtime to the cli input option and the relevant sentiment is displayed. Below i detail the pros and cons of both of the libraries tested.

### To format:
```bash
make format
``` 

### To lint:
```bash
make lint
```

### To run:
```bash
cargo run infer --input <sentence>
```

## Rust-BERT:

Rust-BERT is a natural language processing crate that is a good library of ready to use models and pipelines. I did not fine tune any of the models and from the minimal documentation i understand that it can be a simple BERT-uncased model created by porting Hugging Face's transformers library using onnx runtime.
My original idea for this project was to use a pretrained model-> fine tune it in python -> convert to an onnx file -> execute the inference in Rust using onnx runtime. Here i observed the size of the model file after onnx went u significantly where i was expecting it have an opposite effect. On reading further about this i found that there are additional considerations required to make the size go down or stay the same as the pytorch model. This will be a domain i will continue research in, and maybe implement it one of my future projects.
Here i have used the Sentiment pipeline and the Sentiment model provided by the same.


### Sample Output:
![image](https://github.com/NehaBardeDUKE/AIPI_561_Individual_Project_1/assets/110474064/ef4e46a7-e53e-4b47-812d-1939e3133ca4)
![image](https://github.com/NehaBardeDUKE/AIPI_561_Individual_Project_1/assets/110474064/bdfd6d82-5c31-4f58-90d5-ba03a40161e7)

### My take:
This model provides a polarity score for the sentiment which provides a bit of interpretability wrt to the sentiment of the given sentence.
The actual zero shot sentiment analysis is fairly decent for sentences with triple or double negatives, extremely positive or extremely negative words but if you just pass sentences like "How r u" , it tends to classify this as either positive or negative with a high polarity score. Ideally the polarity score should be low irrespective of how this is classified (signifying neutral sentiment if any).
Another thing that was a bit difficult to navigate (especially since i am still learning Rust) was that the sentiment model only took an array of sentences and not a single sentence. I had to come up with a way to handle this but that kind of flexibility to inut any number of sentences would be very welcome.

## Rust NLTK:

This is a toolkit created for Rust using the python nltk toolkit. It claims to provide all the models that the NLTK library provides in python, however i did not find the Vader lexicon here, which was a bummer as i have found it to have given me great results for zero shot infernce in the past and it would have been beneficial for this side by side comparison.
Here i used the "sentiment" model which takes as an input just 1 sentence and the language that the model will be using. 


### Sample Output:
![image](https://github.com/NehaBardeDUKE/AIPI_561_Individual_Project_1/assets/110474064/d2060858-a0db-4f6f-a267-48349a85c917)
![image](https://github.com/NehaBardeDUKE/AIPI_561_Individual_Project_1/assets/110474064/bb7d04b4-5ebf-4c31-9da4-b0746e15ca18)


### My take:
The model output is not interpretable or explainable at all. I tried to find what the said class output actually means but i couldnt find that documentation anywhere. From just putting in random sentences i think class 0 means negative sentiment, 1 means neutral sentiment and 2 means positive sentiment. However again there are no polarity scores or probability of the class provided so it is not the most reliable model where you would want to explain the results. Additionally the documentation doesnt specify what is the return type of the result, so it can be a bit tricky to figure out how you want to display the same.
This model however was very easy to implement and could easily be assimilated into my current code without having to change a lot of things.
However, it requires you to install some packages beforehand using python and i personally wouldnt want to do that if i am planning on using Rust end to end.

## Future enhancements:

1. The original code i wrote for Rust-Bert sentiment model was using the rust-bert pipeline version 0.20.0 but on the 3rd of June 2023, they upgraded the rust-bert pipeline to the next version 0.21.0 which causes a lot of issues with the torch-sys dependencies. There is a lot of manual workaround for this. As a continuation of the work done here, i will try to automate those manual steps so that someone will be able to simply pull the project and integrate it in their work.
2. The Rust NLTK library has always given me a lot of trouble when i tried to dockerize it. There continues be an issue with the dependencies when you try to dockerize it especially with the cc linker and some pytorch libraries. I will continue working on this.
3. Create a package for easy integration.
4. Try to finetune a model and make new model deployments using gihub actions for CI/CD.





