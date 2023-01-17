#include "oboe/OboeExt.h"

namespace oboe {
  /*void AudioStreamBuilder_init(AudioStreamBuilder *builder) {
    new (builder) AudioStreamBuilder();
  }

  void AudioStreamBuilder_drop(AudioStreamBuilder *builder) {
    builder->~AudioStreamBuilder();
  }*/

  void AudioStreamBuilder_create(AudioStreamBuilder *builder) {
    new(builder) AudioStreamBuilder(); // call constructor on preallocated data buffer
  }

  void AudioStreamBuilder_delete(AudioStreamBuilder *builder) {
    builder->~AudioStreamBuilder(); // call destructor directly to avoid free data
  }

  AudioApi AudioStreamBuilder_getAudioApi(const AudioStreamBuilder *builder) {
    return builder->getAudioApi();
  }

  void AudioStreamBuilder_setAudioApi(AudioStreamBuilder *builder, AudioApi api) {
    builder->setAudioApi(api);
  }

  /// Takes ownership of context (drop_context will be called to free it).
  void AudioStreamBuilder_setCallback(AudioStreamBuilder *builder,
                                      void *context,
                                      const DropContextHandler drop_context,
                                      const AudioReadyHandler audio_ready,
                                      const ErrorCloseHandler before_close,
                                      const ErrorCloseHandler after_close) {
    auto s = std::make_shared<AudioStreamCallbackWrapper>(
        context,
        drop_context,
        audio_ready,
        before_close,
        after_close);

    builder->setDataCallback(s);
    builder->setErrorCallback(s);
  }

  AudioStreamBase* AudioStreamBuilder_getBase(AudioStreamBuilder *builder) {
    return static_cast<AudioStreamBase*>(builder);
  }

  Result AudioStreamBuilder_openStreamShared(AudioStreamBuilder *builder,
                                             AudioStreamShared *sharedStream) {
    new(sharedStream) std::shared_ptr<AudioStream>(); // call constructor on preallocated data buffer
    return builder->openStream(*sharedStream);
  }
}
