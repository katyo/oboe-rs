#ifndef OBOE_EXT_H
#define OBOE_EXT_H

#include "oboe/Oboe.h"

namespace oboe {
  const char *convertAudioStreamToText(AudioStream *oboeStream);

  void AudioStreamBuilder_Initialize(AudioStreamBuilder *builder);

  Result AudioStream_open(AudioStream *oboeStream);
  Result AudioStream_requestStart(AudioStream *oboeStream);
  Result AudioStream_requestPause(AudioStream *oboeStream);
  Result AudioStream_requestFlush(AudioStream *oboeStream);
  Result AudioStream_requestStop(AudioStream *oboeStream);
  StreamState AudioStream_getState(const AudioStream *oboeStream);
  Result AudioStream_waitForStateChange(const AudioStream *oboeStream,
                                        StreamState inputState,
                                        StreamState *nextState,
                                        int64_t timeoutNanoseconds);
  ResultWithValue<int32_t>
  AudioStream_setBufferSizeInFrames(AudioStream *oboeStream,
                                    int32_t requestedFrames);
  ResultWithValue<int32_t>
  AudioStream_getXRunCount(const AudioStream *oboeStream);
  bool AudioStream_isXRunCountSupported(const AudioStream *oboeStream);
  int32_t AudioStream_getFramesPerBurst(AudioStream *oboeStream);
  ResultWithValue<double>
  AudioStream_calculateLatencyMillis(AudioStream *oboeStream);
  AudioApi AudioStream_getAudioApi(const AudioStream *oboeStream);

  typedef DataCallbackResult (*AudioReadyHandler)(void *context,
                                                  AudioStream *oboeStream,
                                                  void *audioData,
                                                  int32_t numFrames);

  typedef void (*ErrorCloseHandler)(void *context,
                                    AudioStream *oboeStream,
                                    Result error);

  class AudioStreamCallbackWrapper: AudioStreamCallback {
  public:
    AudioStreamCallbackWrapper(void *context,
                               AudioReadyHandler audio_ready,
                               ErrorCloseHandler before_close,
                               ErrorCloseHandler after_close);

    DataCallbackResult onAudioReady(AudioStream *oboeStream,
                                    void *audioData,
                                    int32_t numFrames);

    void onErrorBeforeClose(AudioStream *oboeStream,
                            Result error);

    void onErrorAfterClose(AudioStream *oboeStream,
                           Result error);

  private:
    void *_context;
    AudioReadyHandler _audio_ready;
    ErrorCloseHandler _before_close;
    ErrorCloseHandler _after_close;
  };
}

#endif
