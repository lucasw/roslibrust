#ifndef SENSOR_MSGS_MESSAGE_CAMERAINFO
#define SENSOR_MSGS_MESSAGE_CAMERAINFO

#include <string>
#include <vector>
#include <memory>

#include <ros/types.h>
#include <ros/serialization.h>
#include <ros/builtin_message_traits.h>
#include <ros/message_operations.h>

#include <std_msgs/Header.h>
#include <sensor_msgs/RegionOfInterest.h>

namespace sensor_msgs {

template <class ContainerAllocator>
struct CameraInfo_
{
  typedef CameraInfo_<ContainerAllocator> Type;
  CameraInfo_()
    : header()
    , height()
    , width()
    , distortion_model()
    , D()
    , K()
    , R()
    , P()
    , binning_x()
    , binning_y()
    , roi() {
  }

  CameraInfo_(const ContainerAllocator& _alloc)
    : header(_alloc)
    , height(_alloc)
    , width(_alloc)
    , distortion_model(_alloc)
    , D(_alloc)
    , K(_alloc)
    , R(_alloc)
    , P(_alloc)
    , binning_x(_alloc)
    , binning_y(_alloc)
    , roi(_alloc) {
    (void)_alloc;
  }
    
        typedef ::std_msgs::Header_<ContainerAllocator> _header_type;
    _header_type header;
    
        typedef uint32_t _height_type;
    _height_type height;
    
        typedef uint32_t _width_type;
    _width_type width;
    
        typedef std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> _distortion_model_type;
    _distortion_model_type distortion_model;
    
        typedef std::vector<double, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<double>> _D_type;
    _D_type D;
    
        typedef std::vector<double, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<double>> _K_type;
    _K_type K;
    
        typedef std::vector<double, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<double>> _R_type;
    _R_type R;
    
        typedef std::vector<double, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<double>> _P_type;
    _P_type P;
    
        typedef uint32_t _binning_x_type;
    _binning_x_type binning_x;
    
        typedef uint32_t _binning_y_type;
    _binning_y_type binning_y;
    
        typedef ::sensor_msgs::RegionOfInterest_<ContainerAllocator> _roi_type;
    _roi_type roi;

  

  typedef boost::shared_ptr< ::sensor_msgs::CameraInfo_<ContainerAllocator>> Ptr;
  typedef boost::shared_ptr< ::sensor_msgs::CameraInfo_<ContainerAllocator> const> ConstPtr;

}; // struct CameraInfo_

typedef ::sensor_msgs::CameraInfo_<std::allocator<void>> CameraInfo;

typedef boost::shared_ptr< ::sensor_msgs::CameraInfo> CameraInfoPtr;
typedef boost::shared_ptr< ::sensor_msgs::CameraInfo const> CameraInfoConstPtr;

// constants requiring out of line definition

template<typename ContainerAllocator>
std::ostream& operator<<(std::ostream& s, const ::sensor_msgs::CameraInfo_<ContainerAllocator> & v)
{
ros::message_operations::Printer< ::sensor_msgs::CameraInfo_<ContainerAllocator> >::stream(s, "", v);
return s;
}

template<typename ContainerAllocator1, typename ContainerAllocator2>
bool operator==(const ::sensor_msgs::CameraInfo_<ContainerAllocator1> & lhs, const ::sensor_msgs::CameraInfo_<ContainerAllocator2> & rhs)
{
  return
    lhs.header == rhs.header &&
    lhs.height == rhs.height &&
    lhs.width == rhs.width &&
    lhs.distortion_model == rhs.distortion_model &&
    lhs.D == rhs.D &&
    lhs.K == rhs.K &&
    lhs.R == rhs.R &&
    lhs.P == rhs.P &&
    lhs.binning_x == rhs.binning_x &&
    lhs.binning_y == rhs.binning_y &&
    lhs.roi == rhs.roi &&
    true;
}

template<typename ContainerAllocator1, typename ContainerAllocator2>
bool operator!=(const ::sensor_msgs::CameraInfo_<ContainerAllocator1> & lhs, const ::sensor_msgs::CameraInfo_<ContainerAllocator2> & rhs)
{
  return !(lhs == rhs);
}


} // namespace sensor_msgs

namespace ros
{
namespace message_traits
{
template <class ContainerAllocator>
struct IsMessage< ::sensor_msgs::CameraInfo_<ContainerAllocator>>
  : TrueType
  { };

template <class ContainerAllocator>
struct IsMessage< ::sensor_msgs::CameraInfo_<ContainerAllocator> const>
  : TrueType
  { };

template <class ContainerAllocator>
struct IsFixedSize< ::sensor_msgs::CameraInfo_<ContainerAllocator>>
  : FalseType
    { };

template <class ContainerAllocator>
struct IsFixedSize< ::sensor_msgs::CameraInfo_<ContainerAllocator> const>
  : FalseType
    { };

template <class ContainerAllocator>
struct HasHeader< ::sensor_msgs::CameraInfo_<ContainerAllocator>>
  : TrueType
    { };

template <class ContainerAllocator>
struct HasHeader< ::sensor_msgs::CameraInfo_<ContainerAllocator> const>
  : TrueType
    { };

template<class ContainerAllocator>
struct MD5Sum< ::sensor_msgs::CameraInfo_<ContainerAllocator>>
{
  static const char* value()
  {
    return "0b90a09f7d964437a2b7ac1f61cd712f";
  }

  static const char* value(const ::sensor_msgs::CameraInfo_<ContainerAllocator>&) { return value(); }
  static const uint64_t static_value1 = 0x0b90a09f7d964437ULL;
  static const uint64_t static_value2 = 0xa2b7ac1f61cd712fULL;
};

template<class ContainerAllocator>
struct DataType< ::sensor_msgs::CameraInfo_<ContainerAllocator>>
{
  static const char* value()
  {
    return "sensor_msgs/CameraInfo";
  }

  static const char* value(const ::sensor_msgs::CameraInfo_<ContainerAllocator>&) { return value(); }
};

template<class ContainerAllocator>
struct Definition< ::sensor_msgs::CameraInfo_<ContainerAllocator>>
{
  static const char* value()
  {
    return "";
  }

  static const char* value(const ::sensor_msgs::CameraInfo_<ContainerAllocator>&) { return value(); }
};

} // namespace message_traits
} // namespace ros

namespace ros
{
namespace serialization
{

template<class ContainerAllocator>
struct Serializer< ::sensor_msgs::CameraInfo_<ContainerAllocator>>
{
  template<typename Stream, typename T> inline static void allInOne(Stream& stream, T m)
  {
    stream.next(m.header);
    stream.next(m.height);
    stream.next(m.width);
    stream.next(m.distortion_model);
    stream.next(m.D);
    stream.next(m.K);
    stream.next(m.R);
    stream.next(m.P);
    stream.next(m.binning_x);
    stream.next(m.binning_y);
    stream.next(m.roi);
  }

  ROS_DECLARE_ALLINONE_SERIALIZER
}; // struct CameraInfo_

} // namespace serialization
} // namespace ros

namespace ros
{
namespace message_operations
{

template<class ContainerAllocator>
struct Printer< ::sensor_msgs::CameraInfo_<ContainerAllocator>>
{
  template<typename Stream> static void stream(Stream& s, const std::string& indent, const ::sensor_msgs::CameraInfo_<ContainerAllocator>& v)
  {

    s << indent << "header: ";
    s << std::endl;
    Printer< ::std_msgs::Header_<ContainerAllocator>>::stream(s, indent + "  ", v.header);

    s << indent << "height: ";
    Printer< uint32_t>::stream(s, indent + "  ", v.height);

    s << indent << "width: ";
    Printer< uint32_t>::stream(s, indent + "  ", v.width);

    s << indent << "distortion_model: ";
    Printer< std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>>::stream(s, indent + "  ", v.distortion_model);
    
    s << indent << "D[]" << std::endl;
    for (size_t i = 0; i < v.D.size(); ++i)
    {
      s << indent << "  D[" << i << "]: ";
      Printer<double>::stream(s, indent + "  ", v.D[i]);
    }
    
    s << indent << "K[]" << std::endl;
    for (size_t i = 0; i < v.K.size(); ++i)
    {
      s << indent << "  K[" << i << "]: ";
      Printer<double>::stream(s, indent + "  ", v.K[i]);
    }
    
    s << indent << "R[]" << std::endl;
    for (size_t i = 0; i < v.R.size(); ++i)
    {
      s << indent << "  R[" << i << "]: ";
      Printer<double>::stream(s, indent + "  ", v.R[i]);
    }
    
    s << indent << "P[]" << std::endl;
    for (size_t i = 0; i < v.P.size(); ++i)
    {
      s << indent << "  P[" << i << "]: ";
      Printer<double>::stream(s, indent + "  ", v.P[i]);
    }

    s << indent << "binning_x: ";
    Printer< uint32_t>::stream(s, indent + "  ", v.binning_x);

    s << indent << "binning_y: ";
    Printer< uint32_t>::stream(s, indent + "  ", v.binning_y);

    s << indent << "roi: ";
    s << std::endl;
    Printer< ::sensor_msgs::RegionOfInterest_<ContainerAllocator>>::stream(s, indent + "  ", v.roi);
  }
};

} // namespace message_operations
} // namespace ros

#endif // SENSOR_MSGS_MESSAGE_CAMERAINFO