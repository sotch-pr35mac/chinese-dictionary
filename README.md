# chinese-dictionary

[![Build Status](https://travis-ci.com/sotch-pr35mac/chinese-dictionary.svg?branch=master)](https://travis-ci.com/sotch-pr35mac/chinese-dictionary)

### About

A searchable Chinese / English dictionary with helpful utilities.

### Features
- Search with Traditional Chinese characters, Simplified Chinese characters, pinyin with tone marks, pinyin with tone numbers, pinyin with no tones, and English.
- Classify a string of text as either English, Pinyin, or Chinese characters.
- Convert between Traditional and Simplified Chinese characters.
- Segment strings of Chinese characters into tokens using a dictionary-driven segmentation approach.

### Installation

```bash
npm install chinese-dictionary --save
```

### Usage

Querying the dictionary
```js
const dictionary = require('chinese-dictionary');

dictionary.init().then(() => { // Initialization may take a while
	dictionary.query('test').then(result => {
		// Learn more about the dictionary entry format below
		console.log(result[0].simplified); // --> '实验'
	});
});
```

Classying a string of text
```js
const dictionary = require('chinese-dictionary');

dictionary.init().then(() => { // Initialization may take a while
	dictionary.classify('test').then(result => {
		// There are four possible return options
		// from the classify function:
		// 'EN' --> Represents the text was in English
		// 'PY' --> Represents the text was in Pinyin
		// 'ZH' --> Represents the text was in Chinese Characters
		// 'UN' --> Represents the classification result was uncertain
		console.log(result); // --> 'EN'
	});
});
```

Convert between Traditional and Simplified Chinese characters
```js
const dictionary = require('chinese-dictionary');

dictionary.init().then(() => { // Initialization may take a while
	dictionary.convertToTraditional('实验').then(result => console.log(result)); // --> 實驗
	dictionary.convertToSimplified('實驗').then(result => console.log(result)); // --> 实验

	console.log(dictionary.isSimplified('实验')); // --> true
	console.log(dictionary.isTraditional('實驗')); // --> true
});
```

Segment a string of characters
```js
const dictionary = require('chinese-dictionary');

dictionary.init().then(() => { // Initialization may take a while
	dictionary.segment('今天天气不错').then(result => console.log(result)); // --> ['今天', '天气', '不错']
});
```

### Dictionary Entry Format
```js
{
	traditional: '天氣',
	simplified: '天气',
	pinyinMarks: 'tiān qì',
	pinyinNumbers: 'tian1 qi4',
	english: ['weather'],
	toneMarks: [1, 4],
	hash: '999999999...',
	hsk: 1,
	word_id: 123456,
	measureWords: [
		{
			traditional: '個',
			simplified: '个',
			pinyinMarks: 'gè',
			pinyinNumbers: 'ge4'
		}
	]
}
```

### License
This software is licensed under the [MIT License](https://github.com/sotch-pr35mac/chinese-dictionary/blob/master/LICENSE).
