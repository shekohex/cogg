// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: cpp-socket/userping.proto

#ifndef PROTOBUF_INCLUDED_cpp_2dsocket_2fuserping_2eproto
#define PROTOBUF_INCLUDED_cpp_2dsocket_2fuserping_2eproto

#include <string>

#include <google/protobuf/stubs/common.h>

#if GOOGLE_PROTOBUF_VERSION < 3006001
#error This file was generated by a newer version of protoc which is
#error incompatible with your Protocol Buffer headers.  Please update
#error your headers.
#endif
#if 3006001 < GOOGLE_PROTOBUF_MIN_PROTOC_VERSION
#error This file was generated by an older version of protoc which is
#error incompatible with your Protocol Buffer headers.  Please
#error regenerate this file with a newer version of protoc.
#endif

#include <google/protobuf/io/coded_stream.h>
#include <google/protobuf/arena.h>
#include <google/protobuf/arenastring.h>
#include <google/protobuf/generated_message_table_driven.h>
#include <google/protobuf/generated_message_util.h>
#include <google/protobuf/inlined_string_field.h>
#include <google/protobuf/metadata.h>
#include <google/protobuf/message.h>
#include <google/protobuf/repeated_field.h>  // IWYU pragma: export
#include <google/protobuf/extension_set.h>  // IWYU pragma: export
#include <google/protobuf/unknown_field_set.h>
// @@protoc_insertion_point(includes)
#define PROTOBUF_INTERNAL_EXPORT_protobuf_cpp_2dsocket_2fuserping_2eproto 

namespace protobuf_cpp_2dsocket_2fuserping_2eproto {
// Internal implementation detail -- do not use these members.
struct TableStruct {
  static const ::google::protobuf::internal::ParseTableField entries[];
  static const ::google::protobuf::internal::AuxillaryParseTableField aux[];
  static const ::google::protobuf::internal::ParseTable schema[1];
  static const ::google::protobuf::internal::FieldMetadata field_metadata[];
  static const ::google::protobuf::internal::SerializationTable serialization_table[];
  static const ::google::protobuf::uint32 offsets[];
};
void AddDescriptors();
}  // namespace protobuf_cpp_2dsocket_2fuserping_2eproto
namespace cogg {
class UserPing;
class UserPingDefaultTypeInternal;
extern UserPingDefaultTypeInternal _UserPing_default_instance_;
}  // namespace cogg
namespace google {
namespace protobuf {
template<> ::cogg::UserPing* Arena::CreateMaybeMessage<::cogg::UserPing>(Arena*);
}  // namespace protobuf
}  // namespace google
namespace cogg {

// ===================================================================

class UserPing : public ::google::protobuf::Message /* @@protoc_insertion_point(class_definition:cogg.UserPing) */ {
 public:
  UserPing();
  virtual ~UserPing();

  UserPing(const UserPing& from);

  inline UserPing& operator=(const UserPing& from) {
    CopyFrom(from);
    return *this;
  }
  #if LANG_CXX11
  UserPing(UserPing&& from) noexcept
    : UserPing() {
    *this = ::std::move(from);
  }

  inline UserPing& operator=(UserPing&& from) noexcept {
    if (GetArenaNoVirtual() == from.GetArenaNoVirtual()) {
      if (this != &from) InternalSwap(&from);
    } else {
      CopyFrom(from);
    }
    return *this;
  }
  #endif
  static const ::google::protobuf::Descriptor* descriptor();
  static const UserPing& default_instance();

  static void InitAsDefaultInstance();  // FOR INTERNAL USE ONLY
  static inline const UserPing* internal_default_instance() {
    return reinterpret_cast<const UserPing*>(
               &_UserPing_default_instance_);
  }
  static constexpr int kIndexInFileMessages =
    0;

  void Swap(UserPing* other);
  friend void swap(UserPing& a, UserPing& b) {
    a.Swap(&b);
  }

  // implements Message ----------------------------------------------

  inline UserPing* New() const final {
    return CreateMaybeMessage<UserPing>(NULL);
  }

  UserPing* New(::google::protobuf::Arena* arena) const final {
    return CreateMaybeMessage<UserPing>(arena);
  }
  void CopyFrom(const ::google::protobuf::Message& from) final;
  void MergeFrom(const ::google::protobuf::Message& from) final;
  void CopyFrom(const UserPing& from);
  void MergeFrom(const UserPing& from);
  void Clear() final;
  bool IsInitialized() const final;

  size_t ByteSizeLong() const final;
  bool MergePartialFromCodedStream(
      ::google::protobuf::io::CodedInputStream* input) final;
  void SerializeWithCachedSizes(
      ::google::protobuf::io::CodedOutputStream* output) const final;
  ::google::protobuf::uint8* InternalSerializeWithCachedSizesToArray(
      bool deterministic, ::google::protobuf::uint8* target) const final;
  int GetCachedSize() const final { return _cached_size_.Get(); }

  private:
  void SharedCtor();
  void SharedDtor();
  void SetCachedSize(int size) const final;
  void InternalSwap(UserPing* other);
  private:
  inline ::google::protobuf::Arena* GetArenaNoVirtual() const {
    return NULL;
  }
  inline void* MaybeArenaPtr() const {
    return NULL;
  }
  public:

  ::google::protobuf::Metadata GetMetadata() const final;

  // nested types ----------------------------------------------------

  // accessors -------------------------------------------------------

  // string username = 2;
  void clear_username();
  static const int kUsernameFieldNumber = 2;
  const ::std::string& username() const;
  void set_username(const ::std::string& value);
  #if LANG_CXX11
  void set_username(::std::string&& value);
  #endif
  void set_username(const char* value);
  void set_username(const char* value, size_t size);
  ::std::string* mutable_username();
  ::std::string* release_username();
  void set_allocated_username(::std::string* username);

  // string client_hash = 3;
  void clear_client_hash();
  static const int kClientHashFieldNumber = 3;
  const ::std::string& client_hash() const;
  void set_client_hash(const ::std::string& value);
  #if LANG_CXX11
  void set_client_hash(::std::string&& value);
  #endif
  void set_client_hash(const char* value);
  void set_client_hash(const char* value, size_t size);
  ::std::string* mutable_client_hash();
  ::std::string* release_client_hash();
  void set_allocated_client_hash(::std::string* client_hash);

  // int64 timestamp = 4;
  void clear_timestamp();
  static const int kTimestampFieldNumber = 4;
  ::google::protobuf::int64 timestamp() const;
  void set_timestamp(::google::protobuf::int64 value);

  // uint32 packetId = 1;
  void clear_packetid();
  static const int kPacketIdFieldNumber = 1;
  ::google::protobuf::uint32 packetid() const;
  void set_packetid(::google::protobuf::uint32 value);

  // @@protoc_insertion_point(class_scope:cogg.UserPing)
 private:

  ::google::protobuf::internal::InternalMetadataWithArena _internal_metadata_;
  ::google::protobuf::internal::ArenaStringPtr username_;
  ::google::protobuf::internal::ArenaStringPtr client_hash_;
  ::google::protobuf::int64 timestamp_;
  ::google::protobuf::uint32 packetid_;
  mutable ::google::protobuf::internal::CachedSize _cached_size_;
  friend struct ::protobuf_cpp_2dsocket_2fuserping_2eproto::TableStruct;
};
// ===================================================================


// ===================================================================

#ifdef __GNUC__
  #pragma GCC diagnostic push
  #pragma GCC diagnostic ignored "-Wstrict-aliasing"
#endif  // __GNUC__
// UserPing

// uint32 packetId = 1;
inline void UserPing::clear_packetid() {
  packetid_ = 0u;
}
inline ::google::protobuf::uint32 UserPing::packetid() const {
  // @@protoc_insertion_point(field_get:cogg.UserPing.packetId)
  return packetid_;
}
inline void UserPing::set_packetid(::google::protobuf::uint32 value) {
  
  packetid_ = value;
  // @@protoc_insertion_point(field_set:cogg.UserPing.packetId)
}

// string username = 2;
inline void UserPing::clear_username() {
  username_.ClearToEmptyNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
inline const ::std::string& UserPing::username() const {
  // @@protoc_insertion_point(field_get:cogg.UserPing.username)
  return username_.GetNoArena();
}
inline void UserPing::set_username(const ::std::string& value) {
  
  username_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), value);
  // @@protoc_insertion_point(field_set:cogg.UserPing.username)
}
#if LANG_CXX11
inline void UserPing::set_username(::std::string&& value) {
  
  username_.SetNoArena(
    &::google::protobuf::internal::GetEmptyStringAlreadyInited(), ::std::move(value));
  // @@protoc_insertion_point(field_set_rvalue:cogg.UserPing.username)
}
#endif
inline void UserPing::set_username(const char* value) {
  GOOGLE_DCHECK(value != NULL);
  
  username_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), ::std::string(value));
  // @@protoc_insertion_point(field_set_char:cogg.UserPing.username)
}
inline void UserPing::set_username(const char* value, size_t size) {
  
  username_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(),
      ::std::string(reinterpret_cast<const char*>(value), size));
  // @@protoc_insertion_point(field_set_pointer:cogg.UserPing.username)
}
inline ::std::string* UserPing::mutable_username() {
  
  // @@protoc_insertion_point(field_mutable:cogg.UserPing.username)
  return username_.MutableNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
inline ::std::string* UserPing::release_username() {
  // @@protoc_insertion_point(field_release:cogg.UserPing.username)
  
  return username_.ReleaseNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
inline void UserPing::set_allocated_username(::std::string* username) {
  if (username != NULL) {
    
  } else {
    
  }
  username_.SetAllocatedNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), username);
  // @@protoc_insertion_point(field_set_allocated:cogg.UserPing.username)
}

// string client_hash = 3;
inline void UserPing::clear_client_hash() {
  client_hash_.ClearToEmptyNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
inline const ::std::string& UserPing::client_hash() const {
  // @@protoc_insertion_point(field_get:cogg.UserPing.client_hash)
  return client_hash_.GetNoArena();
}
inline void UserPing::set_client_hash(const ::std::string& value) {
  
  client_hash_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), value);
  // @@protoc_insertion_point(field_set:cogg.UserPing.client_hash)
}
#if LANG_CXX11
inline void UserPing::set_client_hash(::std::string&& value) {
  
  client_hash_.SetNoArena(
    &::google::protobuf::internal::GetEmptyStringAlreadyInited(), ::std::move(value));
  // @@protoc_insertion_point(field_set_rvalue:cogg.UserPing.client_hash)
}
#endif
inline void UserPing::set_client_hash(const char* value) {
  GOOGLE_DCHECK(value != NULL);
  
  client_hash_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), ::std::string(value));
  // @@protoc_insertion_point(field_set_char:cogg.UserPing.client_hash)
}
inline void UserPing::set_client_hash(const char* value, size_t size) {
  
  client_hash_.SetNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(),
      ::std::string(reinterpret_cast<const char*>(value), size));
  // @@protoc_insertion_point(field_set_pointer:cogg.UserPing.client_hash)
}
inline ::std::string* UserPing::mutable_client_hash() {
  
  // @@protoc_insertion_point(field_mutable:cogg.UserPing.client_hash)
  return client_hash_.MutableNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
inline ::std::string* UserPing::release_client_hash() {
  // @@protoc_insertion_point(field_release:cogg.UserPing.client_hash)
  
  return client_hash_.ReleaseNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited());
}
inline void UserPing::set_allocated_client_hash(::std::string* client_hash) {
  if (client_hash != NULL) {
    
  } else {
    
  }
  client_hash_.SetAllocatedNoArena(&::google::protobuf::internal::GetEmptyStringAlreadyInited(), client_hash);
  // @@protoc_insertion_point(field_set_allocated:cogg.UserPing.client_hash)
}

// int64 timestamp = 4;
inline void UserPing::clear_timestamp() {
  timestamp_ = GOOGLE_LONGLONG(0);
}
inline ::google::protobuf::int64 UserPing::timestamp() const {
  // @@protoc_insertion_point(field_get:cogg.UserPing.timestamp)
  return timestamp_;
}
inline void UserPing::set_timestamp(::google::protobuf::int64 value) {
  
  timestamp_ = value;
  // @@protoc_insertion_point(field_set:cogg.UserPing.timestamp)
}

#ifdef __GNUC__
  #pragma GCC diagnostic pop
#endif  // __GNUC__

// @@protoc_insertion_point(namespace_scope)

}  // namespace cogg

// @@protoc_insertion_point(global_scope)

#endif  // PROTOBUF_INCLUDED_cpp_2dsocket_2fuserping_2eproto