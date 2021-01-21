extern crate chinese_dictionary;

use neon::prelude::*;
use std::sync::Once;
use chinese_dictionary::ChineseDictionary;
use chinese_dictionary::ClassificationResult::*;
use chinese_dictionary::WordEntry;
use chinese_dictionary::MeasureWord;

static INIT: Once = Once::new();
static mut DICTIONARY: Option<ChineseDictionary> = None;

#[macro_export]
macro_rules! convert_measure_word {
	($context:expr, $measure_word:expr) => {
		{
			let measure_word_js = JsObject::new(&mut $context);
			
			let traditional = $context.string(&$measure_word.traditional);
			let simplified = $context.string(&$measure_word.simplified);
			let pinyin_marks = $context.string(&$measure_word.pinyin_marks);
			let pinyin_numbers = $context.string(&$measure_word.pinyin_numbers);

			measure_word_js.set(&mut $context, "traditional", traditional).unwrap();
			measure_word_js.set(&mut $context, "simplified", simplified).unwrap();
			measure_word_js.set(&mut $context, "pinyinMarks", pinyin_marks).unwrap();
			measure_word_js.set(&mut $context, "pinyinNumbers", pinyin_numbers).unwrap();

			measure_word_js
		}
	};
}

#[macro_export]
macro_rules! convert_word_entry {
	($context:expr, $word_entry:expr) => {
		{
			let word = JsObject::new(&mut $context);

			let traditional = $context.string(&$word_entry.traditional);
			let simplified = $context.string(&$word_entry.simplified);
			let pinyin_marks = $context.string(&$word_entry.pinyin_marks);
			let pinyin_numbers = $context.string(&$word_entry.pinyin_numbers);
			let hash = $context.string(format!("{}", $word_entry.hash));
			let hsk = $context.number($word_entry.hsk as u32);
			let word_id = $context.number($word_entry.word_id);

		
			let english = JsArray::new(&mut $context, $word_entry.english.len() as u32);
			for (i, definition) in $word_entry.english.iter().enumerate() {
				let definition_js = $context.string(definition);
				english.set(&mut $context, i as u32, definition_js).unwrap();
			}
		
			let measure_words = JsArray::new(&mut $context, $word_entry.measure_words.len() as u32);
			for (i, measure_word) in $word_entry.measure_words.iter().enumerate() {
				let measure_word_js = convert_measure_word!($context, measure_word);
				measure_words.set(&mut $context, i as u32, measure_word_js).unwrap();
			}
		
			let tone_marks = JsArray::new(&mut $context, $word_entry.tone_marks.len() as u32);
			for (i, tone_mark) in $word_entry.tone_marks.iter().enumerate() {
				let tone_mark_js = $context.number(*tone_mark as u32);
				tone_marks.set(&mut $context, i as u32, tone_mark_js).unwrap();
			}

			word.set(&mut $context, "traditional", traditional).unwrap();
			word.set(&mut $context, "simplified", simplified).unwrap();
			word.set(&mut $context, "pinyinMarks", pinyin_marks).unwrap();
			word.set(&mut $context, "pinyinNumbers", pinyin_numbers).unwrap();
			word.set(&mut $context, "hash", hash).unwrap();
			word.set(&mut $context, "hsk", hsk).unwrap();
			word.set(&mut $context, "wordId", word_id).unwrap();
			word.set(&mut $context, "english", english).unwrap();
			word.set(&mut $context, "measureWords", measure_words).unwrap();
			word.set(&mut $context, "toneMarks", tone_marks).unwrap();
		
			word
		}
	}
}

fn init(mut cx: FunctionContext) -> JsResult<JsUndefined> {
	unsafe {
		INIT.call_once(|| {
			DICTIONARY = Some(ChineseDictionary::new());
		});
	}
	return Ok(cx.undefined());
}

fn classify(mut cx: FunctionContext) -> JsResult<JsString> {
	let input = cx.argument::<JsString>(0)?.value();
	unsafe {
		return match &DICTIONARY {
			None => panic!("You must call init before calling this function."),
			Some(dictionary) => {
				Ok(cx.string(match dictionary.classify(&input) {
					EN => "EN",
					PY => "PY",,
					ZH => "ZH",
					_ => "UN"
				}))
			}
		}
	}
}

fn convert_to_simplified(mut cx: FunctionContext) -> JsResult<JsString> {
	let input = cx.argument::<JsString>(0)?.value();
	unsafe {
		return match &DICTIONARY {
			None => panic!("You must call init before calling this function."),
			Some(dictionary) => {
				Ok(cx.string(dictionary.convert_to_simplified(&input)))
			}
		}
	}
}

fn convert_to_traditional(mut cx: FunctionContext) -> JsResult<JsString> {
	let input = cx.argument::<JsString>(0)?.value();
	unsafe {
		return match &DICTIONARY {
			None => panic!("You must call init before calling this function."),
			Some(dictionary) => {
				Ok(cx.string(dictionary.convert_to_traditional(&input)))
			}
		}
	}
}

fn is_traditional(mut cx: FunctionContext) -> JsResult<JsBoolean> {
	let input = cx.argument::<JsString>(0)?.value();
	unsafe {
		return match &DICTIONARY {
			None => panic!("You must call init before calling this function."),
			Some(dictionary) => {
				Ok(cx.boolean(dictionary.is_traditional(&input)))
			}
		}
	}
}

fn is_simplified(mut cx: FunctionContext) -> JsResult<JsBoolean> {
	let input = cx.argument::<JsString>(0)?.value();
	unsafe {
		return match &DICTIONARY {
			None => panic!("You must call init before calling this function."),
			Some(dictionary) => {
				Ok(cx.boolean(dictionary.is_simplified(&input)))
			}
		}
	}
}

fn segment(mut cx: FunctionContext) -> JsResult<JsArray> {
	let input = cx.argument::<JsString>(0)?.value();
	unsafe {
		return match &DICTIONARY {
			None => panic!("You must call init before calling this function."),
			Some(dictionary) => {
				let segments: Vec<String> = dictionary.segment(&input);
				let js_array = JsArray::new(&mut cx, segments.len() as u32);
				for (i, obj) in segments.iter().enumerate() {
					let js_string = cx.string(obj);
					js_array.set(&mut cx, i as u32, js_string).unwrap();
				}
				return Ok(js_array);
			}
		}
	}
}

fn query_by_english(mut cx: FunctionContext) -> JsResult<JsArray> {
	let input = cx.argument::<JsString>(0)?.value().to_lowercase();
	unsafe {
		return match &DICTIONARY {
			None => panic!("You must call init before calling this function."),
			Some(dictionary) => {
				let words: Vec<&WordEntry> = dictionary.query_by_english(&input);
				let js_array = JsArray::new(&mut cx, words.len() as u32);
				for (i, word_entry) in words.iter().enumerate() {
					let word = convert_word_entry!(cx, word_entry);
					js_array.set(&mut cx, i as u32, word).unwrap();
				}
				return Ok(js_array);
			}
		}
	}
}

fn query_by_pinyin(mut cx: FunctionContext) -> JsResult<JsArray> {
	let input = cx.argument::<JsString>(0)?.value().to_lowercase();
	unsafe {
		return match &DICTIONARY {
			None => panic!("You must call init before calling this function."),
			Some(dictionary) => {
				let words: Vec<&WordEntry> = dictionary.query_by_pinyin(&input);
				let js_array = JsArray::new(&mut cx, words.len() as u32);
				for (i, word_entry) in words.iter().enumerate() {
					let word = convert_word_entry!(cx, word_entry);
					js_array.set(&mut cx, i as u32, word).unwrap();
				}
				return Ok(js_array);
			}
		}
	}
}

fn query_by_chinese(mut cx: FunctionContext) -> JsResult<JsArray> {
	let input = cx.argument::<JsString>(0)?.value().to_lowercase();
	unsafe {
		return match &DICTIONARY {
			None => panic!("You must call init before calling this function."),
			Some(dictionary) => {
				let words: Vec<&WordEntry> = dictionary.query_by_chinese(&input);
				let js_array = JsArray::new(&mut cx, words.len() as u32);
				for (i, word_entry) in words.iter().enumerate() {
					let word = convert_word_entry!(cx, word_entry);
					js_array.set(&mut cx, i as u32, word).unwrap();
				}
				return Ok(js_array);
			}
		}
	}
}

fn query(mut cx: FunctionContext) -> JsResult<JsArray> {
	let input = cx.argument::<JsString>(0)?.value().to_lowercase();
	unsafe {
		return match &DICTIONARY {
			None => panic!("You must call init before calling this function."),
			Some(dictionary) => {
				let words: Vec<&WordEntry> = match dictionary.query(&input) {
					None => vec![],
					Some(result) => result
				};
				let js_array = JsArray::new(&mut cx, words.len() as u32);
				for (i, word_entry) in words.iter().enumerate() {
					let word = convert_word_entry!(cx, word_entry);
					js_array.set(&mut cx, i as u32, word).unwrap();
				}
				return Ok(js_array);
			}
		}
	}
}

register_module!(mut cx, {
	cx.export_function("init", init)?;
	cx.export_function("classify", classify)?;
	cx.export_function("convertToSimplified", convert_to_simplified)?;
	cx.export_function("convertToTraditional", convert_to_traditional)?;
	cx.export_function("isTraditional", is_traditional)?;
	cx.export_function("isSimplified", is_simplified)?;
	cx.export_function("segment", segment)?;
	cx.export_function("queryByEnglish", query_by_english)?;
	cx.export_function("queryByPinyin", query_by_pinyin)?;
	cx.export_function("queryByChinese", query_by_chinese)?;
	cx.export_function("query", query)?;
	Ok(())
});
