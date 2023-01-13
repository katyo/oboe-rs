#include "oboe/OboeExt.h"

#include <android/log.h>

namespace oboe {
  AudioStreamCallbackWrapper::
  AudioStreamCallbackWrapper(void *context,
                             const DropContextHandler drop_context,
                             const AudioReadyHandler audio_ready,
                             const ErrorCloseHandler before_close,
                             const ErrorCloseHandler after_close):
    _context(context),
    _drop_context(drop_context),
    _audio_ready(audio_ready),
    _before_close(before_close),
    _after_close(after_close) {}

  AudioStreamCallbackWrapper
   ::~AudioStreamCallbackWrapper() {
     _drop_context(_context);
  }

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
