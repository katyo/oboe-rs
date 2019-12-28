#include "oboe/OboeExt.h"

namespace oboe {
  AudioStreamCallbackWrapper::
  AudioStreamCallbackWrapper(const AudioReadyHandler audio_ready,
                             const ErrorCloseHandler before_close,
                             const ErrorCloseHandler after_close):
    AudioStreamCallback(),
    _context(nullptr),
    _audio_ready(audio_ready),
    _before_close(before_close),
    _after_close(after_close) {}

  void AudioStreamCallbackWrapper::setContext(void *context) {
    _context = context;
  }

  DataCallbackResult AudioStreamCallbackWrapper::
  onAudioReady(AudioStream *oboeStream,
               void *audioData,
               int32_t numFrames) {
    return _audio_ready(&_context, oboeStream, audioData, numFrames);
  }

  void AudioStreamCallbackWrapper::
  onErrorBeforeClose(AudioStream *oboeStream,
                     Result error) {
    _before_close(&_context, oboeStream, error);
  }

  void AudioStreamCallbackWrapper::
  onErrorAfterClose(AudioStream *oboeStream,
                    Result error) {
    _after_close(&_context, oboeStream, error);
  }
}
