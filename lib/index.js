const dictionary = require('../native');
const returnPromise = (func, value) => {
	return new Promise((resolve, reject) => {
		try {
			let result = value ? func(value) : func();
			resolve(result);
		} catch(error) {
			reject(error);
		}
	});
};

module.exports = {
	init: () => returnPromise(dictionary.init),
	query: query => returnPromise(dictionary.query, query),
	queryByChinese: query => returnPromise(dictionary.queryByChinese, query),
	queryByPinyin: query => returnPromise(dictionary.queryByPinyin, query),
	queryByEnglish: query => returnPromise(dictionary.queryByEnglish, query),
	segment: sentence => returnPromise(dictionary.segment, sentence),
	isSimplified: text => dictionary.isSimplified(text),
	isTraditional: text => dictionary.isTraditional(text),
	convertToTraditional: text => returnPromise(dictionary.convertToTraditional, text),
	convertToSimplified: text => returnPromise(dictionary.convertToSimplified, text),
	classify: text => returnPromise(dictionary.classify, text)
};
