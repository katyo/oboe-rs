#include "oboe/OboeExt.h"

namespace oboe {
  /*void AudioStreamBuilder_init(AudioStreamBuilder *builder) {
    new (builder) AudioStreamBuilder();
  }

  void AudioStreamBuilder_drop(AudioStreamBuilder *builder) {
    builder->~AudioStreamBuilder();
  }*/

  AudioStreamBuilder *AudioStreamBuilder_new() {
    return new AudioStreamBuilder();
  }

  void AudioStreamBuilder_delete(AudioStreamBuilder *builder) {
    delete builder;
  }

  AudioApi AudioStreamBuilder_getAudioApi(const AudioStreamBuilder *builder) {
    return builder->getAudioApi();
  }

  void AudioStreamBuilder_setAudioApi(AudioStreamBuilder *builder, AudioApi api) {
    builder->setAudioApi(api);
  }

  void AudioStreamBuilder_setCallback(AudioStreamBuilder *builder,
                                      AudioStreamCallbackWrapper *callback) {
    builder->setCallback(callback);
  }

  AudioStreamBase* AudioStreamBuilder_getBase(AudioStreamBuilder *builder) {
    return static_cast<AudioStreamBase*>(builder);
  }
}
