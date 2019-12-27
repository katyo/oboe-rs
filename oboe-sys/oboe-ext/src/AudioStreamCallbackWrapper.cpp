#include "oboe/OboeExt.h"

namespace oboe {
  AudioStreamCallbackWrapper::
  AudioStreamCallbackWrapper(void *context,
                             AudioReadyHandler audio_ready,
                             ErrorCloseHandler before_close,
                             ErrorCloseHandler after_close):
    AudioStreamCallback(),
    _context(context),
    _audio_ready(audio_ready),
    _before_close(before_close),
    _after_close(after_close) {}

  DataCallbackResult AudioStreamCallbackWrapper::
  onAudioReady(AudioStream *oboeStream,
               void *audioData,
               int32_t numFrames) {
    return _audio_ready(_context, oboeStream, audioData, numFrames);
  }

  void AudioStreamCallbackWrapper::
  onErrorBeforeClose(AudioStream *oboeStream,
                     Result error) {
    _before_close(_context, oboeStream, error);
  }

  void AudioStreamCallbackWrapper::
  onErrorAfterClose(AudioStream *oboeStream,
                    Result error) {
    _after_close(_context, oboeStream, error);
  }
}
