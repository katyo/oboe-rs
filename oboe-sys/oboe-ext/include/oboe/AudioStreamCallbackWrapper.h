#ifndef AUDIO_STREAM_CALLBACK_H
#define AUDIO_STREAM_CALLBACK_H

namespace oboe {
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
